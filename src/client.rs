use std::sync::Arc;

use bevy::{platform::sync::Mutex, prelude::*};
use discord_presence::models::EventData;
use quork::traits::list::ListVariants;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct Client(discord_presence::Client);

#[derive(Message, Debug, Deref, DerefMut)]
pub struct RpcEvent(EventData);

/// A queue of Discord events to be processed by Bevy
// we need to do this because discord-presence runs event handlers on a separate thread unmanaged by bevy
#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct EventQueue(Arc<Mutex<Vec<EventData>>>);

impl Client {
    /// Create a new Discord RPC client
    pub(crate) fn new(client_id: u64) -> Self {
        let client = discord_presence::Client::new(client_id);
        Self(client)
    }
}

pub(crate) fn startup(mut client: ResMut<Client>, event_queue: ResMut<EventQueue>) {
    // forward all events to bevy
    for event in discord_presence::Event::VARIANTS {
        _ = client.on_event(event, {
            let queue = event_queue.clone();
            move |ctx| {
                let _ = queue.lock().as_mut().map(|q| q.push(ctx.event));
            }
        });
    }

    // start the client
    _ = client.start();
}

pub(crate) fn drain_events(queue: Res<EventQueue>, mut writer: MessageWriter<RpcEvent>) {
    let _ = queue
        .lock()
        .as_mut()
        .map(|queued| queued.drain(..).map(|e| writer.write(RpcEvent(e))));
}
