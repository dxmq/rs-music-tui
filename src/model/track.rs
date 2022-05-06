use std::time::Duration;

use crate::model::album::Album;
use crate::model::artist::Artist;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: usize,
    pub name: String,
    #[serde(alias = "ar")]
    pub artists: Vec<Artist>,
    #[serde(alias = "al")]
    pub album: Album,
    #[serde(alias = "dt")]
    pub duration: usize,
    pub fee: usize,
    #[serde(alias = "popularity")]
    pub pop: f32,
    // pub resource_state: bool,
    // pub publish_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackUrl {
    pub id: usize,
    pub url: Option<String>,
    pub br: usize,
    // 1vip收费，0免费
    pub fee: usize,
    pub free_trial_info: Option<FreeTrialInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreeTrialInfo {
    pub start: usize,
    pub end: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrackUrlResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<TrackUrl>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedTracks {
    #[serde(default)]
    pub daily_songs: Vec<Track>,
    #[serde(default)]
    pub order_songs: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecommendedTracksResp {
    pub code: usize,
    pub data: Option<RecommendedTracks>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LyricResp {
    pub code: usize,
    pub lrc: Lrc,
    pub tlyric: Lrc,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lrc {
    pub lyric: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyric {
    pub lyric: String,
    pub timeline: Duration,
}
