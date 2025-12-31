use bevy::prelude::*;
use discord_presence::Client as DiscordClient;

#[derive(Resource)]
struct Client(DiscordClient);

#[non_exhaustive]
pub struct DiscordRpcPlugin {
    client_id: u64
}

impl DiscordRpcPlugin {
    pub fn new(client_id: u64) -> Self {
        Self { client_id }
    }
}

impl Plugin for DiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        // instantiate the client
        let client = Client(DiscordClient::new(self.client_id));
        app.insert_resource(client);
    }
}