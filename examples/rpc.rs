use bevy::prelude::*;
use bevy_discord_rpc::{DiscordRpcPlugin, RpcActivity, RpcEvent};

fn read_events(mut events: MessageReader<RpcEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

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
                .default_activity(
                    RpcActivity::builder()
                        .state("Testing out bevy-discord-rpc")
                        .build(),
                )
                .build(),
        )
        .add_systems(Update, read_events)
        .run();
}
