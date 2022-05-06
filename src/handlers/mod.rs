pub use input::handler as input_handler;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::{IoEvent, Key};
use crate::handlers::search::SearchResultBlock;
use crate::model::enums::{PlayingItem, ToggleState};

pub(crate) mod common_key_events;
pub(crate) mod empty;
pub(crate) mod error_screen;
pub(crate) mod help_menu;
pub(crate) mod home;
pub(crate) mod input;
pub(crate) mod library;
pub(crate) mod lyric;
pub(crate) mod my_playlist;
pub(crate) mod playbar;
pub(crate) mod search;
mod search_results;
mod subscribe_playlist;
pub(crate) mod track_table;
mod dialog;

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
        _ if key == app.user_config.keys.jump_to_context => {}
        _ if key == app.user_config.keys.toggle_playback => {
            app.toggle_playback();
        }
        _ if key == app.user_config.keys.next_track => {
            app.next_or_prev_track(ToggleState::Next);
        }
        _ if key == app.user_config.keys.previous_track => {
            app.next_or_prev_track(ToggleState::Prev);
        }
        _ if key == app.user_config.keys.repeat => {
            app.toggle_play_state();
        }
        _ if key == app.user_config.keys.decrease_volume => {
            app.decrease_volume();
        }
        _ if key == app.user_config.keys.increase_volume => {
            app.increase_volume();
        }
        _ if key == app.user_config.keys.show_lyric => {
            if let Some(context) = app.current_playback_context.clone() {
                if let Some(item) = &context.item {
                    match item {
                        PlayingItem::Track(track) => {
                            app.dispatch(IoEvent::GetLyric(track.id));
                        }
                    }
                }
            }
        }
        _ if key == app.user_config.keys.seek_forwards => {
            app.dispatch(IoEvent::SeekForwards)
        }
        _ if key == app.user_config.keys.seek_backwards => {
            app.dispatch(IoEvent::SeekBackForwards)
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
        // 我创建的歌单
        ActiveBlock::MyPlaylists => {
            my_playlist::handler(key, app);
        }
        // 我的收藏的歌单
        ActiveBlock::SubscribedPlaylists => {
            subscribe_playlist::handler(key, app);
        }
        ActiveBlock::TrackTable => {
            track_table::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::PlayBar => {
            playbar::handler(key, app);
        }
        ActiveBlock::Lyric => {
            lyric::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            search_results::handler(key, app);
        }
        ActiveBlock::Dialog(_) => {
            dialog::handler(key, app);
        }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::SearchResultBlock => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
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
