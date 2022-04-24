use crate::app::App;
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::model::context::TrackTableContext;
use crate::model::table::TrackTable;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index = common_key_events::on_middle_press_handler(&app.track_table.tracks);
            app.track_table.selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&app.track_table.tracks);
            app.track_table.selected_index = next_index;
        }
        Key::Enter => {
            on_enter(app);
        }
        _ => {}
    }
}

fn on_enter(app: &mut App) {
    let TrackTable {
        context,
        selected_index,
        tracks,
    } = &app.track_table;
    match &context {
        Some(context) => match context {
            TrackTableContext::MyPlaylists => {
                if let Some(track) = tracks.get(*selected_index) {
                    // let playlist_id = match (&app.active_playlist_index, &app.playlists) {
                    //     (Some(active_playlist_index), Some(playlists)) => playlists
                    //         .get(active_playlist_index.to_owned())
                    //         .map(|selected_playlist| selected_playlist.id.to_owned()),
                    //     _ => None,
                    // };
                    app.dispatch(IoEvent::StartPlayback(track.clone()));
                };
            }
            _ => {}
        },
        None => {}
    };
}
