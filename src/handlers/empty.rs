use crate::app::{ActiveBlock, App};
use crate::event::Key;
use crate::handlers::common_key_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Enter => {
            let current_hovered = app.get_current_route().hovered_block;
            app.set_current_route_state(Some(current_hovered), None);
        }
        k if common_key_events::down_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Library => {
                app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            }
            ActiveBlock::MyPlaylists => {
                app.set_current_route_state(None, Some(ActiveBlock::SubscribedPlaylists))
            }
            ActiveBlock::Lyric
            | ActiveBlock::Artists
            | ActiveBlock::ArtistDetail
            | ActiveBlock::Home
            | ActiveBlock::SubscribedPlaylists
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::PlayBar));
            }
            _ => {}
        },
        k if common_key_events::up_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::SubscribedPlaylists => {
                app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            }
            ActiveBlock::MyPlaylists => {
                app.set_current_route_state(None, Some(ActiveBlock::Library));
            }
            ActiveBlock::PlayBar => {
                app.set_current_route_state(None, Some(ActiveBlock::SubscribedPlaylists));
            }
            _ => {}
        },
        k if common_key_events::left_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Artists
            | ActiveBlock::ArtistDetail
            | ActiveBlock::Home
            | ActiveBlock::TrackTable
            | ActiveBlock::Lyric => {
                app.set_current_route_state(None, Some(ActiveBlock::Library));
            }
            _ => {}
        },
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        _ => (),
    };
}
