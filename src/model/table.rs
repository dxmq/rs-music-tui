#[allow(unused)]
use crate::model::context::TrackTableContext;
use crate::model::track::Track;

#[derive(Default, Clone)]
pub struct TrackTable {
    pub tracks: Vec<Track>,
    pub selected_index: usize,
    pub context: Option<TrackTableContext>,
}

#[derive(Default, Clone)]
pub struct RecentlyPlayed {
    pub tracks: Vec<Track>,
    pub selected_index: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedResp {
    pub code: usize,
    #[serde(default)]
    pub data: RecentlyPlayedData,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedData {
    pub total: usize,
    pub list: Vec<RecentlyPlayedDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedDetail {
    pub resource_id: String,
    pub play_time: u64,
    pub resource_type: String,
    pub data: Track,
}

pub enum TableId {
    #[allow(unused)]
    Album,
    #[allow(unused)]
    AlbumList,
    #[allow(unused)]
    Artist,
    #[allow(unused)]
    Podcast,
    #[allow(unused)]
    Song,
    #[allow(unused)]
    RecentlyPlayed,
    #[allow(unused)]
    MadeForYou,
    #[allow(unused)]
    PodcastEpisodes,
}

#[derive(PartialEq)]
pub enum ColumnId {
    None,
    Title,
    Liked,
}

impl Default for ColumnId {
    fn default() -> Self {
        ColumnId::None
    }
}

pub struct TableHeader<'a> {
    pub id: TableId,
    pub items: Vec<TableHeaderItem<'a>>,
}

impl TableHeader<'_> {
    pub fn get_index(&self, id: ColumnId) -> Option<usize> {
        self.items.iter().position(|item| item.id == id)
    }
}

#[derive(Default)]
pub struct TableHeaderItem<'a> {
    pub id: ColumnId,
    pub text: &'a str,
    pub width: u16,
}

#[derive(Debug)]
pub struct TableItem {
    pub id: usize,
    pub format: Vec<String>,
}
