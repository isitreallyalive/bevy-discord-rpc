use bevy::prelude::*;
pub use discord_presence::models::{
    ActivityAssets, ActivityParty, ActivitySecrets, ActivityType, DisplayType,
};
use discord_presence::{Event, models::EventData};

use crate::{
    activity::ActivityData,
    client::{Client, EventQueue},
};
pub use activity::{Activity, Timestamps};

mod activity;
mod client;

#[derive(bon::Builder)]
pub struct DiscordRpcPlugin {
    #[builder(start_fn)]
    client_id: u64,
    activity: Option<ActivityData>,
}

#[derive(Message, Debug)]
pub struct RpcEvent {
    event: Event,
    #[allow(dead_code)]
    data: EventData,
}

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
