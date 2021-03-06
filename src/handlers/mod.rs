pub use input::handler as input_handler;
pub use login::login_button_handler;
pub use login::password_input_handler;
pub use login::phone_input_handler;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::{IoEvent, Key};
use crate::handlers::search::SearchResultBlock;
use crate::model::artist::ArtistBlock;
use crate::model::enums::ToggleState;
use crate::model::login::LoginState;

mod album_tracks;
mod artist_detail;
mod artists;
pub(crate) mod common_key_events;
mod dialog;
pub(crate) mod empty;
pub(crate) mod error_screen;
pub(crate) mod help_menu;
pub(crate) mod home;
pub(crate) mod input;
pub(crate) mod library;
mod login;
pub(crate) mod lyric;
pub(crate) mod my_playlist;
pub(crate) mod playbar;
pub(crate) mod search;
mod search_results;
mod subscribe_playlist;
pub(crate) mod track_table;

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
        _ if key == app.user_config.keys.reset_play => {
            app.dispatch(IoEvent::ResetPlay);
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
                    app.dispatch(IoEvent::GetLyric(item.id, true));
                }
            }
        }
        _ if key == app.user_config.keys.show_playbar_lyric => {
            app.is_show_playbar_lyric = !app.is_show_playbar_lyric;
        }
        _ if key == app.user_config.keys.seek_forwards => app.dispatch(IoEvent::SeekForwards),
        _ if key == app.user_config.keys.seek_backwards => app.dispatch(IoEvent::SeekBackForwards),
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
        // ??????????????????
        ActiveBlock::MyPlaylists => {
            my_playlist::handler(key, app);
        }
        // ?????????????????????
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
        ActiveBlock::Artists => {
            artists::handler(key, app);
        }
        ActiveBlock::ArtistDetail => {
            artist_detail::handler(key, app);
        }
        ActiveBlock::AlbumTracks => {
            album_tracks::handler(key, app);
        }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::SearchResultBlock => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        ActiveBlock::ArtistDetail => {
            if let Some(artist) = &mut app.artist_detail {
                artist.artist_detail_selected_block = ArtistBlock::Empty;
            }
        }
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        ActiveBlock::Dialog(_) => {
            app.pop_navigation_stack();
        }
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}

pub fn handle_app_login_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
pub fn handle_app_login(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            handle_app_login_escape(app);
        }
        _ => {
            let current_route = app.get_current_route();
            if current_route.active_block == ActiveBlock::Empty {
                match key {
                    Key::Enter => {
                        let current_hovered = app.get_current_route().hovered_block;
                        app.set_current_route_state(Some(current_hovered), None);
                    }
                    k if common_key_events::down_event2(k) => {
                        match app.get_current_route().hovered_block {
                            ActiveBlock::PhoneBlock => {
                                app.set_current_route_state(None, Some(ActiveBlock::PasswordBlock));
                            }
                            ActiveBlock::PasswordBlock => {
                                app.set_current_route_state(
                                    Some(ActiveBlock::LoginButton),
                                    Some(ActiveBlock::LoginButton),
                                );
                                app.login_info.login_state = LoginState::Confirm;
                            }
                            _ => {}
                        }
                    }
                    k if common_key_events::up_event(k) => {
                        if app.get_current_route().hovered_block == ActiveBlock::PasswordBlock {
                            app.set_current_route_state(None, Some(ActiveBlock::PhoneBlock));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
