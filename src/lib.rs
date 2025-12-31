use bevy::prelude::*;
use discord_presence::models::{EventData};

pub use crate::activity::RpcActivity;
use crate::{client::{Client, EventQueue}};

mod activity;
mod client;

#[derive(bon::Builder)]
#[non_exhaustive]
pub struct DiscordRpcPlugin {
    #[builder(start_fn)]
    client_id: u64,
    default_activity: Option<RpcActivity>,
}

#[derive(Message, Debug, Deref, DerefMut)]
pub struct RpcEvent(EventData);

/// Stores the default activity to set on startup
#[derive(Resource, Deref)]
struct DefaultActivity(RpcActivity);

impl Plugin for DiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        // instantiate the client
        app.insert_resource(Client::new(self.client_id))
            .insert_resource(EventQueue::default())
            .add_message::<RpcActivity>()
            .add_message::<RpcEvent>()
            .add_systems(Startup, client::startup)
            .add_systems(Update, (client::apply_activity, client::drain_events));

        // set default activity if provided
        if let Some(activity) = self.default_activity.clone() {
            app.insert_resource(DefaultActivity(activity));
        }
    }
}
