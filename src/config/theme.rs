use serde::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserTheme {
    pub active: Option<String>,
    pub banner: Option<String>,
    pub error_border: Option<String>,
    pub error_text: Option<String>,
    pub hint: Option<String>,
    pub hovered: Option<String>,
    pub inactive: Option<String>,
    pub playbar_background: Option<String>,
    pub playbar_progress: Option<String>,
    pub playbar_progress_text: Option<String>,
    pub playbar_text: Option<String>,
    pub selected: Option<String>,
    pub text: Option<String>,
    pub header: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub analysis_bar: Color,
    pub analysis_bar_text: Color,
    pub active: Color,
    pub banner: Color,
    pub error_border: Color,
    pub error_text: Color,
    pub hint: Color,
    pub hovered: Color,
    pub inactive: Color,
    pub playbar_background: Color,
    pub playbar_progress: Color,
    pub playbar_progress_text: Color,
    pub playbar_text: Color,
    pub selected: Color,
    pub text: Color,
    pub header: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            analysis_bar: Color::LightCyan,
            analysis_bar_text: Color::Reset,
            active: Color::LightCyan,
            banner: Color::LightRed,
            error_border: Color::Red,
            error_text: Color::LightRed,
            hint: Color::Yellow,
            // hovered: Color::Magenta,
            hovered: Color::DarkGray,
            inactive: Color::Gray,
            playbar_background: Color::Black,
            playbar_progress: Color::LightRed,
            playbar_progress_text: Color::LightRed,
            playbar_text: Color::Reset,
            selected: Color::LightRed,
            text: Color::Reset,
            header: Color::Reset,
        }
    }
}
