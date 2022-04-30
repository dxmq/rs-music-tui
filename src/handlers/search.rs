use crate::model::track::Track;

#[allow(unused)]
pub enum SearchResult {
    Tracks(Option<Vec<Track>>),
}
