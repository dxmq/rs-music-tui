use serde::{Deserialize, Serialize};

pub enum TableId {
    Album,
    AlbumList,
    Artist,
    Podcast,
    Song,
    RecentlyPlayed,
    MadeForYou,
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

pub struct TableItem {
    pub id: String,
    pub format: Vec<String>,
}
