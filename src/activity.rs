use bevy::prelude::*;
use discord_presence::{
    Result,
    models::{Activity, payload::Payload},
};

use crate::client::Client;

#[derive(Message, bon::Builder, Clone, Debug)]
#[non_exhaustive]
pub struct RpcActivity {
    #[builder(into)]
    state: String
}

impl RpcActivity {
    /// Apply the activity to the given client.
    pub(crate) fn apply(&self, client: &mut Client) -> Result<Payload<Activity>> {
        client.set_activity(|mut a| {
            a.state = Some(self.state.clone());
            a
        })
    }
}
