use bevy::prelude::*;

pub use crate::client::RpcEvent;
use crate::client::{Client, EventQueue};

mod client;

#[non_exhaustive]
pub struct DiscordRpcPlugin {
    client_id: u64,
}

impl DiscordRpcPlugin {
    pub fn new(client_id: u64) -> Self {
        Self { client_id }
    }
}

impl Plugin for DiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        // instantiate the client
        app.insert_resource(Client::new(self.client_id))
            .insert_resource(EventQueue::default())
            .add_message::<RpcEvent>()
            .add_systems(Startup, client::startup)
            .add_systems(Update, client::drain_events);
    }
}
