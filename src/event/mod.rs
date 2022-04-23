pub use self::events::{Event, Events};
pub use self::key::Key;

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
    // 开始播放歌曲
    StartPlayback(Option<usize>, Option<Vec<String>>, Option<usize>),
}
