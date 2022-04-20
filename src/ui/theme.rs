use tui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
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
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            selected: Color::LightCyan,
            hovered: Color::Magenta,
            inactive: Color::Gray,
            hint: Color::Yellow,
            text: Color::Reset,
            banner: Color::LightCyan,
        }
    }
}
