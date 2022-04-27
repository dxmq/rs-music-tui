use crate::app::App;
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::model::table::RecentlyPlayed;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &app.recently_played.tracks,
                Some(app.recently_played.selected_index),
            );
            app.recently_played.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.recently_played.tracks,
                Some(app.recently_played.selected_index),
            );
            app.recently_played.selected_index = next_index;
        }
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.recently_played.selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index =
                common_key_events::on_middle_press_handler(&app.recently_played.tracks);
            app.recently_played.selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&app.recently_played.tracks);
            app.recently_played.selected_index = next_index;
        }
        Key::Enter => {
            let RecentlyPlayed {
                selected_index,
                tracks,
                ..
            } = app.recently_played.clone();
            app.my_play_tracks = app.track_table.clone();
            if let Some(track) = tracks.get(selected_index) {
                app.dispatch(IoEvent::StartPlayback(track.clone()));
            }
        }
        _ => {}
    }
}
