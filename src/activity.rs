use std::{
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use bevy::prelude::*;
use discord_presence::models::{
    ActivityAssets, ActivityParty, ActivitySecrets, ActivityTimestamps,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Timestamps {
    start: Option<u64>,
    end: Option<u64>,
}

impl Timestamps {
    pub fn now() -> Result<Self, SystemTimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        Ok(Self {
            start: Some(now),
            end: None,
        })
    }
}

impl From<Timestamps> for ActivityTimestamps {
    fn from(timestamps: Timestamps) -> Self {
        ActivityTimestamps {
            start: timestamps.start,
            end: timestamps.end,
        }
    }
}

#[derive(bon::Builder, Clone, Debug, Default)]
#[builder(on(String, into))]
#[non_exhaustive]
pub struct ActivityData {
    #[cfg(feature = "unstable_name")]
    pub name: Option<String>,
    /// The player's current party status
    pub state: Option<String>,
    /// What the player is currently doing
    pub details: Option<String>,
    /// Helps create elapsed/remaining timestamps on a player's profile
    pub timestamps: Option<Timestamps>,
    /// Assets to display on the player's profile
    pub assets: Option<ActivityAssets>,
    /// Information about the player's party
    pub party: Option<ActivityParty>,
    /// Secret passwords for joining the player's game
    pub secrets: Option<ActivitySecrets>,
    /// Whether this activity is an instanced context, like a match
    pub instance: Option<bool>,
}

#[derive(Resource)]
pub struct Activity(pub(crate) Option<ActivityData>);

impl Activity {
    /// Create a new activity builder
    pub fn builder() -> ActivityDataBuilder {
        ActivityData::builder()
    }

    /// Create a new empty activity
    pub fn empty() -> Self {
        Self(None)
    }

    /// Replaces the current activity with the given one
    pub fn replace(&mut self, activity: ActivityData) {
        self.0 = Some(activity);
    }

    /// Clears the current activity
    pub fn clear(&mut self) {
        self.0 = None;
    }

    /// Provides a fluent API for updating fields of the activity
    pub fn update<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut ActivityData),
    {
        // initialise if empty
        if self.0.is_none() {
            self.0 = Some(ActivityData::default());
        }

        // we can unwrap safely here
        let activity = self.0.as_mut().unwrap();
        updater(activity);
    }
}

impl From<ActivityData> for Activity {
    fn from(activity: ActivityData) -> Self {
        Self(Some(activity))
    }
}

impl From<Option<ActivityData>> for Activity {
    fn from(activity: Option<ActivityData>) -> Self {
        Self(activity)
    }
}
