use bevy::prelude::*;
use discord_presence::models::EventData;
pub use discord_presence::models::{
    ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps, ActivityType, DisplayType,
};

use crate::{
    activity::ActivityData,
    client::{Client, EventQueue},
};
pub use activity::Activity;

mod activity;
mod client;

#[derive(bon::Builder)]
#[non_exhaustive]
pub struct DiscordRpcPlugin {
    #[builder(start_fn)]
    client_id: u64,
    activity: Option<ActivityData>,
}

#[derive(Message, Debug, Deref, DerefMut)]
pub struct RpcEvent(EventData);

impl Plugin for DiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Client::new(self.client_id))
            .insert_resource(EventQueue::default())
            .insert_resource(Activity::from(self.activity.clone()))
            .add_message::<RpcEvent>()
            // create a connection to discord on startup
            .add_systems(Startup, client::startup)
            .add_systems(
                Update,
                (
                    // drain any queued events
                    client::drain_events,
                    // update the activity if it has changed
                    client::apply_activity,
                ),
            );
    }
}
