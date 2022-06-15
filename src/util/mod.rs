use crate::app::{ActiveBlock, App};
use crate::config::theme::Theme;
use crate::handlers::search::SearchResultBlock;
use crate::model::artist::{Artist, ArtistBlock};
use std::ffi::OsStr;
use std::ops::Add;
use std::path::{Path, PathBuf};
use tui::style::Style;

pub const BASIC_VIEW_HEIGHT: u16 = 6;
pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

// Make better use of space on small terminals
pub fn get_main_layout_margin(app: &App) -> u16 {
    if app.size.height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(theme.selected),
        (false, true) => Style::default().fg(theme.hovered),
        _ => Style::default().fg(theme.inactive),
    }
}

// 获取歌手string，以,分隔
pub fn create_artist_string(artists: &[Artist]) -> String {
    artists
        .iter()
        .map(|artist| artist.name.clone().unwrap())
        .collect::<Vec<String>>()
        .join(", ")
}

// 获取歌手string，以/分隔
pub fn create_artist_string2(artists: &[Artist]) -> String {
    if artists.len() > 2 {
        let ar = &artists[0..2];
        ar.iter()
            .map(|artist| artist.name.clone().unwrap())
            .collect::<Vec<String>>()
            .join("&")
            .add("...$")
    } else {
        artists
            .iter()
            .map(|artist| artist.name.clone().unwrap())
            .collect::<Vec<String>>()
            .join("&")
    }
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn get_music_path(
    url: Option<&str>,
    cache_dir: &anyhow::Result<PathBuf>,
    music_name_prefix: &str,
) -> Option<PathBuf> {
    let mut suffix = "mp3";
    if let Some(url) = url {
        suffix = get_extension_from_filename(url).unwrap();
    }
    let file_name = format!("{}.{}", music_name_prefix, suffix);
    if let Ok(cache_dir) = cache_dir {
        let p = cache_dir.join(file_name);
        return Some(p);
    }
    None
}

// 获取播放的进度，确保进度在0-100之间
pub fn get_track_progress_percentage(song_progress_ms: u128, track_duration_ms: u32) -> u16 {
    let min_perc = 0_f64;
    let track_progress = std::cmp::min(song_progress_ms, track_duration_ms.into());
    let track_perc = (track_progress as f64 / f64::from(track_duration_ms)) * 100_f64;
    min_perc.max(track_perc) as u16
}

// 播放进度字符串表示
pub fn display_track_progress(progress: u128, track_duration: u32) -> String {
    let duration = millis_to_minutes(u128::from(track_duration));
    let progress_display = millis_to_minutes(progress);
    let remaining = millis_to_minutes(u128::from(track_duration).saturating_sub(progress));

    format!("{}/{} (-{})", progress_display, duration, remaining,)
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
    let padding = 3;
    let width = width - padding;
    (f32::from(width) * percentage) as u16
}

pub fn get_search_results_highlight_state(
    app: &App,
    block_to_match: SearchResultBlock,
) -> (bool, bool) {
    let current_route = app.get_current_route();
    (
        app.search_results.selected_block == block_to_match,
        current_route.hovered_block == ActiveBlock::SearchResultBlock
            && app.search_results.hovered_block == block_to_match,
    )
}

pub fn get_artist_highlight_state(app: &App, block_to_match: ArtistBlock) -> (bool, bool) {
    let current_route = app.get_current_route();
    if let Some(artist) = &app.artist_detail {
        let is_hovered = artist.artist_detail_selected_block == block_to_match;
        let is_selected = current_route.hovered_block == ActiveBlock::ArtistDetail
            && artist.artist_detail_hovered_block == block_to_match;
        (is_hovered, is_selected)
    } else {
        (false, false)
    }
}

pub fn millis_to_minutes2(millis: usize) -> String {
    let minutes = millis / 60000;
    let seconds = (millis % 60000) / 1000;
    let seconds_display = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };

    if seconds == 60 {
        format!("{}:00", minutes + 1)
    } else {
        format!("{}:{}", minutes, seconds_display)
    }
}

pub fn millis_to_minutes(millis: u128) -> String {
    let minutes = millis / 60000;
    let seconds = (millis % 60000) / 1000;
    let seconds_display = if seconds < 10 {
        format!("0{}", seconds)
    } else {
        format!("{}", seconds)
    };

    if seconds == 60 {
        format!("{}:00", minutes + 1)
    } else {
        format!("{}:{}", minutes, seconds_display)
    }
}
