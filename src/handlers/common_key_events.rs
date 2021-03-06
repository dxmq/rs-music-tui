use crate::app::{ActiveBlock, App, RouteId};
use crate::event::Key;

pub fn down_event(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j') | Key::Ctrl('n'))
}

pub fn down_event2(key: Key) -> bool {
    matches!(key, Key::Down | Key::Char('j') | Key::Ctrl('n') | Key::Tab)
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

pub enum KeyAction {
    Down,
    Up,
}

pub fn on_down_or_up_press_handler<T>(
    selection_data: &[T],
    selection_index: Option<usize>,
    action: KeyAction,
    offset: usize,
) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                match action {
                    KeyAction::Down => {
                        if offset >= selection_data.len() {
                            return selection_data.len() - 1;
                        }
                        let next_index = selection_index + offset;
                        return if next_index < selection_data.len() {
                            next_index
                        } else {
                            0
                        };
                    }
                    KeyAction::Up => {
                        if offset >= selection_data.len() {
                            return 0;
                        }
                        if selection_index > 0 && selection_index >= offset {
                            return selection_index - offset;
                        }
                        0
                    }
                };
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
    match app.get_current_route().hovered_block {
        ActiveBlock::MyPlaylists | ActiveBlock::SubscribedPlaylists | ActiveBlock::Library => {
            match app.get_current_route().id {
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
                RouteId::Artists => app.set_current_route_state(
                    Some(ActiveBlock::Artists),
                    Some(ActiveBlock::Artists),
                ),
                RouteId::ArtistDetail => app.set_current_route_state(
                    Some(ActiveBlock::ArtistDetail),
                    Some(ActiveBlock::ArtistDetail),
                ),
                RouteId::AlbumTracks => app.set_current_route_state(
                    Some(ActiveBlock::AlbumTracks),
                    Some(ActiveBlock::AlbumTracks),
                ),
                RouteId::Lyric => {
                    app.set_current_route_state(Some(ActiveBlock::Lyric), Some(ActiveBlock::Lyric));
                }
                RouteId::PhoneBlock => {}
                RouteId::PasswordBlock => {}
                RouteId::LoginButton => {}
                RouteId::Error => {}
                RouteId::BasicView => {}
                RouteId::Dialog => {}
            }
        }
        _ => {}
    }
}

pub fn handle_left_event(app: &mut App) {
    // TODO1: This should send you back to either library or playlist based on last selection
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Library));
}
