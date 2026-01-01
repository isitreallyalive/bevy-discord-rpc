use bevy::prelude::*;
use bevy_discord_rpc::{Activity, DiscordRpcPlugin};

fn main() {
    let client_id = std::env::var("APPLICATION_ID")
        .map(|id| {
            id.parse::<u64>()
                .expect("Application ID must be a valid u64")
        })
        .expect("APPLICATION_ID must be set to a valid u64");

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(
            DiscordRpcPlugin::builder(client_id)
                .activity(
                    Activity::builder()
                        // with the `unstable_name` feature, we can override the application's name
                        .name("bevy-discord-rpc")
                        .details("hello!")
                        .build(),
                )
                .build(),
        )
        .run();
}
