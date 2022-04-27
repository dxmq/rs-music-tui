use crate::app::App;
use crate::event::Key;
use crate::handlers::common_key_events;

#[allow(unused)]
pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        _ => {}
    }
}
