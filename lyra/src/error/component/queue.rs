pub mod play {
    #[derive(thiserror::Error, Debug)]
    #[error("loading many tracks failed: {:?}", .0)]
    pub enum LoadTrackProcessManyError {
        Lavalink(#[from] lavalink_rs::error::LavalinkError),
        Query(#[from] QueryError),
    }

    #[derive(thiserror::Error, Debug)]
    pub enum QueryError {
        #[error(transparent)]
        LoadFailed(#[from] crate::error::LoadFailed),
        #[error("no matches found: {}", .0)]
        NoMatches(Box<str>),
        #[error("search results found: {}", .0)]
        SearchResult(Box<str>),
    }

    #[derive(thiserror::Error, Debug)]
    #[error("playing failed: {:?}", .0)]
    pub enum Error {
        RequireUnsuppressed(#[from] crate::error::command::require::UnsuppressedError),
        Respond(#[from] crate::error::command::RespondError),
        RespondOrFollowup(#[from] crate::error::command::RespondOrFollowupError),
        AutoJoinOrCheckInVoiceWithUser(
            #[from] crate::error::command::util::AutoJoinOrCheckInVoiceWithUserError,
        ),
        Lavalink(#[from] lavalink_rs::error::LavalinkError),
    }
}

pub mod repeat {
    #[derive(thiserror::Error, Debug)]
    #[error(transparent)]
    pub enum Error {
        UnrecognisedConnection(#[from] crate::error::UnrecognisedConnection),
        Respond(#[from] crate::error::command::RespondError),
        UpdateNowPlayingMessage(#[from] crate::error::lavalink::UpdateNowPlayingMessageError),
    }
}

pub mod shuffle {
    #[derive(thiserror::Error, Debug)]
    #[error(transparent)]
    pub enum Error {
        Respond(#[from] crate::error::command::RespondError),
        UpdateNowPlayingMessage(#[from] crate::error::lavalink::UpdateNowPlayingMessageError),
    }
}

pub use repeat::Error as RepeatError;
pub use shuffle::Error as ShuffleError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemoveTracksError {
    #[error(transparent)]
    Lavalink(#[from] lavalink_rs::error::LavalinkError),
    #[error(transparent)]
    Respond(#[from] crate::error::command::RespondError),
    #[error(transparent)]
    Followup(#[from] crate::error::command::FollowupError),
    #[error(transparent)]
    DeserialiseBodyFromHttp(#[from] crate::error::core::DeserialiseBodyFromHttpError),
}
