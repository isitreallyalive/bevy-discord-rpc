use bevy::prelude::*;
use bevy_discord_rpc::{Activity, DiscordRpcPlugin, RpcEvent};

fn read_events(mut events: MessageReader<RpcEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

fn update_activity(mut activity: ResMut<Activity>, time: Res<Time>) {
    let elapsed = time.elapsed_secs() as u64;
    activity.state = format!("uptime: {}s", elapsed);
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
        .add_plugins(DiscordRpcPlugin::builder(client_id).activity(Activity::builder().state("uptime: 0s").build()).build())
        .add_systems(Update, read_events)
        .add_systems(FixedUpdate, update_activity)
        .insert_resource(Time::from_seconds(1.))
        .run();
}
