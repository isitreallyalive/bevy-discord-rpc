use bevy::prelude::*;
use bevy_discord_rpc::{Activity, DiscordRpcPlugin, RpcEvent, Timestamps};

fn read_events(mut events: MessageReader<RpcEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

fn update_activity(mut activity: ResMut<Activity>, time: Res<Time>) {
    let elapsed = (time.elapsed_secs() / 60.0) as u64;

    activity.update(|data| {
        data.details = Some(format!("uptime: {elapsed}m"));
    });
}

fn main() -> Result<()> {
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
                        .state("hello from bevy-discord-rpc")
                        .details("uptime: 0m")
                        .timestamps(Timestamps::now()?)
                        .build(),
                )
                .build(),
        )
        // print any incoming events for debugging
        .add_systems(Update, read_events)
        // update the activity every minute
        .add_systems(FixedUpdate, update_activity)
        .insert_resource(Time::from_seconds(60.))
        .run();

    Ok(())
}
