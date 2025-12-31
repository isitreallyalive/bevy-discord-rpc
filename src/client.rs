use std::{sync::Arc};

use bevy::{platform::sync::Mutex, prelude::*};
use discord_presence::models::EventData;
use quork::traits::list::ListVariants;

use crate::RpcEvent;

// todo: handle discord client shutdowns
#[derive(Resource, Deref, DerefMut)]
pub(crate) struct Client {
    #[deref]
    inner: discord_presence::Client,
    ready: bool,
}

#[derive(Resource, bon::Builder, Debug, Clone, Default)]
#[non_exhaustive]
pub struct Activity {
    #[builder(into)]
    pub state: String,
}

/// A queue of Discord events to be processed by Bevy
// we need to do this because discord-presence runs event handlers on a separate thread unmanaged by bevy
#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct EventQueue(Arc<Mutex<Vec<EventData>>>);

impl Client {
    /// Create a new Discord RPC client
    pub(crate) fn new(client_id: u64) -> Self {
        let client = discord_presence::Client::new(client_id);
        Self {
            inner: client,
            ready: false,
        }
    }
}

/// Initialise the Discord RPC client
pub(crate) fn startup(mut client: ResMut<Client>, event_queue: ResMut<EventQueue>) {
    // forward all events to bevy
    for event in discord_presence::Event::VARIANTS {
        client
            .on_event(event, {
                let queue = event_queue.clone();
                move |ctx| {
                    println!("{:?}", ctx);
                    let _ = queue.lock().as_mut().map(|q| q.push(ctx.event));
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
    activity: If<Res<Activity>>,
) {
    // we need to cache whether the activity has changed in case the client isn't ready yet
    if activity.is_changed() {
        *has_changed = true;
    }

    // only apply if the client is ready and the activity has changed
    if !client.ready || !*has_changed {
        return;
    }

    let _ = client.set_activity(|mut a| {
        a.state = Some(activity.state.to_string());
        a
    });
    *has_changed = false;
}

/// Drain queued events into bevy
pub(crate) fn drain_events(
    queue: Res<EventQueue>,
    mut writer: MessageWriter<RpcEvent>,
    mut client: ResMut<Client>,
) {
    _ = queue.lock().as_mut().map(|queued| {
        queued.drain(..).for_each(|event| {
            match event {
                // connection established, so mark the client as ready
                EventData::Ready(_) => client.ready = true,
                // errors are bad, the client is probably disconnected
                EventData::Error(_) => client.ready = false,
                _ => {}
            }

            writer.write(RpcEvent(event));
        })
    });
}
