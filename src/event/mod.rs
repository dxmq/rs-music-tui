pub use self::events::{Event, Events};
pub use self::key::Key;
use ncmapi::types::{Song, SongUrl};

mod events;
mod key;

#[derive(Debug)]
pub enum IoEvent {
    // GetPlaylists,
    GetSearchResults(String),
    UpdateSearchLimits(u32, u32),
    GetPlaylists,
    GetUser,
    GetPlaylistTracks(usize),
    CurrentUserSavedTracksContains(Vec<String>),
    StartPlayback(Song),
}
