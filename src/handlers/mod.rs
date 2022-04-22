use crate::app::{ActiveBlock, App, RouteId};
use crate::event::Key;

pub(crate) mod common_key_events;
pub(crate) mod help_menu;
pub(crate) mod input;
pub(crate) mod library;
pub(crate) mod playlist;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ if key == app.user_config.keys.search => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ if key == app.user_config.keys.help => {
            app.set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        _ if key == app.user_config.keys.basic_view => {
            app.push_navigation_stack(RouteId::BasicView, ActiveBlock::BasicView);
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
        ActiveBlock::Library => {
            library::handles(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu::handler(key, app);
        }
        // 我的歌单
        ActiveBlock::MyPlaylists => {}
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        // ActiveBlock::SearchResultBlock => {
        //     app.search_results.selected_block = SearchResultBlock::Empty;
        // }
        // ActiveBlock::ArtistBlock => {
        //     if let Some(artist) = &mut app.artist {
        //         artist.artist_selected_block = ArtistBlock::Empty;
        //     }
        // }
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        ActiveBlock::Dialog(_) => {
            app.pop_navigation_stack();
        }
        // These are global views that have no active/inactive distinction so do nothing
        // ActiveBlock::SelectDevice | ActiveBlock::Analysis => {}
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
