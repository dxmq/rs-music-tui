use crate::app::{ActiveBlock, App, RouteId};
use crate::event::Key;

pub fn down_event(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j') | Key::Ctrl('n'))
}

pub fn up_event(key: Key) -> bool {
    matches!(key, Key::Up | Key::Char('k') | Key::Ctrl('p'))
}

pub fn left_event(key: Key) -> bool {
    matches!(key, Key::Left | Key::Char('h') | Key::Ctrl('b'))
}

pub fn right_event(key: Key) -> bool {
    matches!(key, Key::Right | Key::Char('l') | Key::Ctrl('f'))
}

pub fn high_event(key: Key) -> bool {
    matches!(key, Key::Char('H'))
}

pub fn middle_event(key: Key) -> bool {
    matches!(key, Key::Char('M'))
}

pub fn low_event(key: Key) -> bool {
    matches!(key, Key::Char('L'))
}

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_high_press_handler() -> usize {
    0
}

pub fn on_middle_press_handler<T>(selection_data: &[T]) -> usize {
    let mut index = selection_data.len() / 2;
    if selection_data.len() % 2 == 0 {
        index -= 1;
    }
    index
}

pub fn on_low_press_handler<T>(selection_data: &[T]) -> usize {
    selection_data.len() - 1
}

pub fn handle_right_event(app: &mut App) {
    if app.get_current_route().hovered_block == ActiveBlock::Library {
        match app.get_current_route().id {
            RouteId::MadeForYou => {
                app.set_current_route_state(
                    Some(ActiveBlock::MadeForYou),
                    Some(ActiveBlock::MadeForYou),
                );
            }
            RouteId::Home => {
                app.set_current_route_state(Some(ActiveBlock::Home), Some(ActiveBlock::Home));
            }
            RouteId::Search => {
                app.set_current_route_state(
                    Some(ActiveBlock::SearchResultBlock),
                    Some(ActiveBlock::SearchResultBlock),
                );
            }
            RouteId::TrackTable => {
                app.set_current_route_state(
                    Some(ActiveBlock::TrackTable),
                    Some(ActiveBlock::SearchResultBlock),
                );
            }
            RouteId::Error => {}
            RouteId::BasicView => {}
            RouteId::Dialog => {}
        }
    }
}

pub fn handle_left_event(app: &mut App) {
    // TODO1: This should send you back to either library or playlist based on last selection
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
}
