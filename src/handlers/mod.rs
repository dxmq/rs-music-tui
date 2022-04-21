use crate::app::{ActiveBlock, App};
use crate::event::Key;

pub(crate) mod input;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        _ if key == app.user_config.keys.search => {
            app.set_current_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }

        _ => handle_block_events(key, app),
    }
}

pub fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();

    match current_route.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        _ => {}
    }
}
