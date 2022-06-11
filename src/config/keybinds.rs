use crate::event::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct KeyBindings {
    pub back: Key,
    pub next_page: Key,
    pub previous_page: Key,
    pub jump_to_start: Key,
    pub jump_to_end: Key,
    pub jump_to_artist_album: Key,
    pub jump_to_artist_detail: Key,
    pub decrease_volume: Key,
    pub increase_volume: Key,
    pub toggle_playback: Key,
    pub seek_backwards: Key,
    pub seek_forwards: Key,
    pub next_track: Key,
    pub previous_track: Key,
    pub help: Key,
    pub repeat: Key,
    pub search: Key,
    pub submit: Key,
    pub basic_view: Key,
    pub add_item_to_queue: Key,
    pub show_lyric: Key,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyBindingsString {
    pub back: Option<String>,
    pub next_page: Option<String>,
    pub previous_page: Option<String>,
    pub jump_to_start: Option<String>,
    pub jump_to_end: Option<String>,
    pub jump_to_artist_album: Option<String>,
    pub jump_to_artist_detail: Option<String>,
    pub decrease_volume: Option<String>,
    pub increase_volume: Option<String>,
    pub toggle_playback: Option<String>,
    pub seek_backwards: Option<String>,
    pub seek_forwards: Option<String>,
    pub next_track: Option<String>,
    pub previous_track: Option<String>,
    pub help: Option<String>,
    pub repeat: Option<String>,
    pub search: Option<String>,
    pub submit: Option<String>,
    pub basic_view: Option<String>,
    pub add_item_to_queue: Option<String>,
    pub show_lyric: Option<String>,
}
