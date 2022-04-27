pub use self::events::{Event, Events};
pub use self::key::Key;
use crate::model::track::Track;

mod events;
mod key;

#[derive(Debug)]
pub enum IoEvent {
    // GetPlaylists,
    GetSearchResults(String),
    UpdateSearchLimits(u32, u32),
    GetPlaylists,
    GetUser,
    GetPlaylistTracks(usize, u32),
    // CurrentUserSavedTracksContains(Vec<String>),
    StartPlayback(Track),
    // GetCurrentPlayback(Track),
    // PausePlayback,
    TogglePlayBack,
    GetRecentlyPlayed(u32),
    GetRecommendTracks,
    DecreaseVolume,
    IncreaseVolume,
}
