use std::sync::Arc;

use bevy::{platform::sync::Mutex, prelude::*};
use discord_presence::models::EventData;
use quork::traits::list::ListVariants;

use crate::{DefaultActivity, RpcActivity, RpcEvent};

// todo: handle discord client shutdowns
#[derive(Resource, Deref, DerefMut)]
pub(crate) struct Client {
    #[deref]
    inner: discord_presence::Client,
    ready: bool,
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
pub(crate) fn startup(
    mut client: ResMut<Client>,
    event_queue: ResMut<EventQueue>,
) {
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
    default_activity: Option<Res<DefaultActivity>>,
    mut commands: Commands,
    mut reader: MessageReader<RpcActivity>,
) {
    if !client.ready {
        return;
    }

    // apply the default activity if it exists
    if let Some(activity) = default_activity {
        _ = activity.apply(&mut client);
        commands.remove_resource::<DefaultActivity>();
    }

    // apply any queued activities
    for activity in reader.read() {
        _ = activity.apply(&mut client);
    }
}

/// Drain queued events into bevy
pub(crate) fn drain_events(
    queue: Res<EventQueue>,
    mut writer: MessageWriter<RpcEvent>,
    mut client: ResMut<Client>,
) {
    _ = queue.lock().as_mut().map(|queued| {
        queued.drain(..).for_each(|e| {
            // if we get a ready event, mark the client as ready
            if matches!(e, EventData::Ready { .. }) {
                client.ready = true;
            }

            writer.write(RpcEvent(e));
        })
    });
}
