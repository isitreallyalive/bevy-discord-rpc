use bevy::prelude::*;
use bevy_discord_rpc::prelude::*;

mod helpers;

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
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(
            DiscordRpcPlugin::builder(helpers::CLIENT_ID)
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
