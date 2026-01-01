use std::sync::Arc;

use bevy::{platform::sync::Mutex, prelude::*};
use discord_presence::Event;
use quork::traits::list::ListVariants;

use crate::{Activity, RpcEvent};

// todo: handle discord client shutdowns
#[derive(Resource, Deref, DerefMut)]
pub(crate) struct Client {
    #[deref]
    inner: discord_presence::Client,
    connected: bool,
}

/// A queue of Discord events to be processed by Bevy
// we need to do this because discord-presence runs event handlers on a separate thread unmanaged by bevy
#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct EventQueue(Arc<Mutex<Vec<RpcEvent>>>);

impl Client {
    pub(crate) fn new(client_id: u64) -> Self {
        let client = discord_presence::Client::new(client_id);
        Self {
            inner: client,
            connected: false,
        }
    }
}

pub(crate) fn startup(mut client: ResMut<Client>, event_queue: ResMut<EventQueue>) {
    // forward all events to bevy
    for event in discord_presence::Event::VARIANTS {
        client
            .on_event(event, {
                let queue = event_queue.clone();
                move |ctx| {
                    let _ = queue.lock().as_mut().map(|q| {
                        q.push(RpcEvent {
                            event,
                            data: ctx.event,
                        })
                    });
                }
            })
            .persist();
    }

    // start the client
    _ = client.start();
}

pub(crate) fn apply_activity(
    mut client: ResMut<Client>,
    mut has_changed: Local<bool>,
    curr_activity: Res<Activity>,
) {
    let Some(activity) = curr_activity.0.as_ref() else {
        return;
    };

    // we need to cache whether the activity has changed in case the client isn't ready yet
    if curr_activity.is_changed() {
        *has_changed = true;
    }

    // only apply if the client is ready and the activity has changed
    if !client.connected || !*has_changed {
        return;
    }

    let _ = client.set_activity(|mut a| {
        #[cfg(feature = "unstable_name")]
        {
            a.name = activity.name.clone();
        }
        a.state = activity.state.clone();
        a.details = activity.details.clone();
        a.timestamps = activity.timestamps.map(Into::into);
        a.assets = activity.assets.clone();
        a.party = activity.party.clone();
        a.secrets = activity.secrets.clone();
        a.instance = activity.instance;
        a
    });
    *has_changed = false;
}

pub(crate) fn drain_events(
    queue: Res<EventQueue>,
    mut writer: MessageWriter<RpcEvent>,
    mut client: ResMut<Client>,
    mut activity: ResMut<Activity>,
) {
    _ = queue.lock().as_mut().map(|queued| {
        queued.drain(..).for_each(|event| {
            match event.event {
                Event::Connected => client.connected = true,
                Event::Disconnected => {
                    client.connected = false;
                    // make sure the activity is reapplied when we reconnect
                    activity.set_changed();
                }
                _ => {}
            }

            writer.write(event);
        })
    });
}
