use tui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub active: Color,
    // 选中的颜色
    pub selected: Color,
    // 未激活的颜色
    pub inactive: Color,
    // 鼠标悬停的颜色
    pub hovered: Color,
    // 提示的颜色
    pub hint: Color,
    // 文本的颜色
    pub text: Color,
    // banner文本的颜色
    pub banner: Color,
    // 播放条的颜色
    pub playbar_text: Color,
    // 播放条背景
    pub playbar_background: Color,
    // 播放条进度颜色
    pub playbar_progress: Color,
    // 播放条进度文件颜色
    pub playbar_progress_text: Color,
    pub header: Color,

    pub error_border: Color,
    pub error_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            active: Color::Cyan,
            selected: Color::LightCyan,
            hovered: Color::Magenta,
            inactive: Color::Gray,
            hint: Color::Yellow,
            text: Color::Reset,
            banner: Color::LightCyan,
            playbar_text: Color::Reset,
            playbar_background: Color::Black,
            playbar_progress: Color::LightCyan,
            playbar_progress_text: Color::LightCyan,
            header: Color::Reset,

            error_border: Color::Red,
            error_text: Color::LightRed,
        }
    }
}
