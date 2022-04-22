use crate::app::{ActiveBlock, App, RouteId};
use crate::event::{IoEvent, Key};

pub(crate) mod common_key_events;
pub(crate) mod empty;
pub(crate) mod error_screen;
pub(crate) mod help_menu;
pub(crate) mod home;
pub(crate) mod input;
pub(crate) mod library;
pub(crate) mod playlist;

use crate::model::enums::Type;
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
        _ if key == app.user_config.keys.jump_to_context => {
            handle_jump_to_context(app);
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
        ActiveBlock::Error => {
            error_screen::handler(key, app);
        }
        // 我的歌单
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
        ActiveBlock::TrackTable => {
            // track_table::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        _ => {}
    }
}

fn handle_jump_to_context(app: &mut App) {
    if let Some(current_playback_context) = &app.current_playback_context {
        if let Some(play_context) = current_playback_context.context.clone() {
            match play_context._type {
                // rspotify::senum::Type::Album => handle_jump_to_album(app),
                // rspotify::senum::Type::Artist => handle_jump_to_artist_album(app),
                Type::Playlist => app.dispatch(IoEvent::GetPlaylistTracks(498339500)),
                _ => {}
            }
        }
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
