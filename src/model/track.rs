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
pub struct TrackUrl {
    pub id: usize,
    pub url: String,
    pub br: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrackUrlResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<TrackUrl>,
}
