use bevy::prelude::*;
use bevy_discord_rpc::DiscordRpcPlugin;

fn main() {
    let client_id = std::env::var("DISCORD_CLIENT_ID")
        .map(|id| id.parse::<u64>().expect("Client ID must be a valid u64"))
        .expect("DISCORD_CLIENT_ID must be set to a valid u64");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DiscordRpcPlugin::new(client_id));
}