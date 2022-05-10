use crate::app::{ActiveBlock, RouteId};
use crate::event::Key;
use crate::handlers::common_key_events;
use crate::handlers::common_key_events::KeyAction;
use crate::{App, IoEvent};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &app.artists,
                Some(app.artists_selected_index),
            );
            app.artists_selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.artists,
                Some(app.artists_selected_index),
            );
            app.artists_selected_index = next_index;
        }
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.artists_selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index = common_key_events::on_middle_press_handler(&app.artists);
            app.artists_selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&app.artists);
            app.artists_selected_index = next_index;
        }
        k if k == app.user_config.keys.next_page => {
            let next_index = common_key_events::on_down_or_up_press_handler(
                &app.artists,
                Some(app.artists_selected_index),
                KeyAction::Down,
                20,
            );
            app.artists_selected_index = next_index;
        }
        k if k == app.user_config.keys.previous_page => {
            let next_index = common_key_events::on_down_or_up_press_handler(
                &app.artists,
                Some(app.artists_selected_index),
                KeyAction::Up,
                20,
            );
            app.artists_selected_index = next_index;
        }
        k if k == app.user_config.keys.jump_to_end => {
            app.artists_selected_index = 0;
        }
        k if k == app.user_config.keys.jump_to_start => {
            app.artists_selected_index = app.artists.len() - 1;
        }
        Key::Enter => {
            let selected_index = app.artists_selected_index;
            let artist = app.artists.get(selected_index);
            if let Some(arist) = artist {
                let id = arist.id;
                app.dispatch(IoEvent::GetArtistDetail(id));
                app.push_navigation_stack(RouteId::ArtistDetail, ActiveBlock::ArtistDetail);
            }
        }
        _ => {}
    }
}
