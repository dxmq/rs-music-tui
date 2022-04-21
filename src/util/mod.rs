use crate::app::App;
use crate::config::theme::Theme;
use crate::model::artist::SimplifiedArtist;
use tui::style::Style;

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
pub fn create_artist_string(artists: &[SimplifiedArtist]) -> String {
    artists
        .iter()
        .map(|artist| artist.name.to_string())
        .collect::<Vec<String>>()
        .join(", ")
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
