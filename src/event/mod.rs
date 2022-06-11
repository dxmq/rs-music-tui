pub use self::events::{Event, Events};
pub use self::key::Key;
use crate::model::album::Album;
use crate::model::login::LoginForm;
use crate::model::track::Track;

mod events;
mod key;

#[derive(Debug)]
pub enum IoEvent {
    GetSearchResults(String),
    UpdateSearchLimits(u32, u32),
    GetPlaylists,
    GetUser,
    GetPlaylistTracks(usize),
    // CurrentUserSavedTracksContains(Vec<String>),
    StartPlayback(Track),
    // GetCurrentPlayback(Track),
    // PausePlayback,
    TogglePlayBack,
    #[allow(unused)]
    GetRecentlyPlayed,
    GetRecommendTracks,
    DecreaseVolume,
    IncreaseVolume,
    GetLikeList,
    GetLyric(usize, bool),
    // 喜欢or不喜欢歌曲
    ToggleLikeTrack(usize),
    ToggleSubscribePlaylist(usize),
    SeekForwards,
    SeekBackForwards,
    WebLog(usize),
    // 获取我收藏的歌手列表
    GetArtistSubList,
    GetArtistDetail(usize, String),
    GetAlbumTracks(Box<Album>),
    // 收藏or取消收藏歌手
    ToggleSubscribeArtist(usize),
    Login(LoginForm),
    AddToQueue(Track),
}
