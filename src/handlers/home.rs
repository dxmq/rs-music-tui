use crate::app::App;
use crate::event::Key;
use crate::handlers::common_key_events;

const LARGE_SCROLL: u16 = 10;
const SMALL_SCROLL: u16 = 1;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            app.home_scroll += SMALL_SCROLL;
        }
        k if common_key_events::up_event(k) => {
            if app.home_scroll > 0 {
                app.home_scroll -= SMALL_SCROLL;
            }
        }
        k if k == app.user_config.keys.next_page => {
            app.home_scroll += LARGE_SCROLL;
        }
        k if k == app.user_config.keys.previous_page => {
            if app.home_scroll > LARGE_SCROLL {
                app.home_scroll -= LARGE_SCROLL;
            } else {
                app.home_scroll = 0;
            }
        }
        _ => {}
    }
}
