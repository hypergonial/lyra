use twilight_cache_inmemory::{InMemoryCache, model::CachedVoiceState};
use twilight_model::id::{Id, marker::GuildMarker};

use twilight_gateway::{Event, MessageSender};

use crate::error::gateway::ProcessResult;

pub trait Process {
    async fn process(self) -> ProcessResult;
}

pub trait SenderAware {
    fn sender(&self) -> &MessageSender;
}

pub trait OptionallyGuildIdAware {
    fn get_guild_id(&self) -> Option<Id<GuildMarker>>;
}

pub trait GuildIdAware {
    fn guild_id(&self) -> Id<GuildMarker>;
}

#[derive(Debug)]
pub struct LastCachedStates {
    pub voice_state: Option<CachedVoiceState>,
}

impl LastCachedStates {
    pub fn new(cache: &InMemoryCache, event: &Event) -> Self {
        let voice_state = match event {
            Event::VoiceStateUpdate(event) => {
                let guild_id = event
                    .guild_id
                    .expect("bots should currently only be able to join guild voice channels");
                cache
                    .voice_state(event.user_id, guild_id)
                    .as_deref()
                    .cloned()
            }
            _ => None,
        };

        Self { voice_state }
    }
}
