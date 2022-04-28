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
        // Scroll down
        // k if k == app.user_config.keys.next_page => {
        //     if let Some(context) = &app.track_table.context {
        //         if context == &TrackTableContext::MyPlaylists {
        //             if let (Some(playlists), Some(selected_playlist_index)) =
        //                 (&app.playlists, &app.selected_playlist_index)
        //             {
        //                 if let Some(selected_playlist) =
        //                     playlists.get(selected_playlist_index.to_owned())
        //                 {
        //                     if let Some(playlist_tracks) = &app.playlist_tracks {
        //                         if app.playlist_offset + app.large_search_limit
        //                             < playlist_tracks.tracks.len() as u32
        //                         {
        //                             app.playlist_offset += app.large_search_limit;
        //                             let playlist_id = selected_playlist.id.to_owned();
        //                             app.dispatch(IoEvent::GetPlaylistTracks(
        //                                 playlist_id,
        //                                 app.playlist_offset,
        //                             ));
        //                         }
        //                     }
        //                 }
        //             };
        //         }
        //     }
        // }
        _ => {}
    }
}

fn on_enter(app: &mut App) {
    let TrackTable {
        context,
        selected_index,
        tracks,
    } = app.track_table.clone();
    let track = tracks.get(selected_index);
    match &context {
        Some(context) => match context {
            TrackTableContext::MyPlaylists => {
                if track.is_some() {
                    let tracks = tracks.clone();
                    app.my_play_tracks = TrackTable {
                        tracks,
                        selected_index,
                        context: Some(TrackTableContext::MyPlaylists),
                    };
                }
            }
            TrackTableContext::RecommendedTracks => {
                if track.is_some() {
                    let tracks = tracks.clone();
                    app.my_play_tracks = TrackTable {
                        tracks,
                        selected_index,
                        context: Some(TrackTableContext::RecommendedTracks),
                    };
                }
            }
            _ => {}
        },
        None => {}
    };
    if let Some(track) = track {
        app.dispatch(IoEvent::StartPlayback(track.clone()));
    }
}
