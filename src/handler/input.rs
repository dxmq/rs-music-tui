use crate::api::IoEvent;
use crate::app::RouteId::Search;
use crate::app::{ActiveBlock, App};
use crate::event::Key;

/// 处理当搜索框激活时候的事件
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Enter => {
            let input_str: String = app.input.iter().collect();
            process_input(input_str, app);
        }
        _ => {}
    }
}

fn process_input(input: String, app: &mut App) {
    if input.is_empty() {
        return;
    }
    // 在搜索曲目时，清除播放列表选择
    app.selected_playlist_index = Some(0);

    app.dispatch(IoEvent::GetSearchResults(input));
    app.push_navigation_stack(Search, ActiveBlock::SearchResultBlock);
}
