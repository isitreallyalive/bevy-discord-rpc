use bevy::prelude::*;
use discord_presence::models::EventData;

pub use crate::client::Activity;
use crate::client::{Client, EventQueue};

mod client;

#[derive(bon::Builder)]
#[non_exhaustive]
pub struct DiscordRpcPlugin {
    #[builder(start_fn)]
    client_id: u64,
    activity: Activity
}

#[derive(Message, Debug, Deref, DerefMut)]
pub struct RpcEvent(EventData);


impl Plugin for DiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        // instantiate the client
        app.insert_resource(Client::new(self.client_id))
            .insert_resource(EventQueue::default())
            .insert_resource(self.activity.clone())
            .add_message::<RpcEvent>()
            .add_systems(Startup, client::startup)
            .add_systems(Update, (client::drain_events, client::apply_activity));
    }
}
