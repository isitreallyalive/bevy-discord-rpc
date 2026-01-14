use bevy::prelude::*;
use bevy_discord_rpc::{Activity, DiscordRpcPlugin};

mod helpers;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(
            DiscordRpcPlugin::builder(helpers::CLIENT_ID)
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
