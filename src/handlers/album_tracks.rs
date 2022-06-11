use super::common_key_events;
use crate::model::context::TrackTableContext;
use crate::model::table::TrackTable;
use crate::{app::App, event::Key, IoEvent};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
        k if common_key_events::down_event(k) => {
            if let Some(album_detail) = &mut app.album_detail {
                let next_index = common_key_events::on_down_press_handler(
                    &album_detail.tracks,
                    Some(album_detail.selected_track_index),
                );
                album_detail.selected_track_index = next_index;
            }
        }
        k if common_key_events::up_event(k) => {
            if let Some(album_detail) = &mut app.album_detail {
                let next_index = common_key_events::on_up_press_handler(
                    &album_detail.tracks,
                    Some(album_detail.selected_track_index),
                );
                album_detail.selected_track_index = next_index;
            }
        }
        k if common_key_events::high_event(k) => handle_high_event(app),
        k if common_key_events::middle_event(k) => handle_middle_event(app),
        k if common_key_events::low_event(k) => handle_low_event(app),
        Key::Enter => {
            if let Some(album_detail) = &app.album_detail.clone() {
                if let Some(track) = album_detail.tracks.get(album_detail.selected_track_index) {
                    app.dispatch(IoEvent::StartPlayback(track.clone()));
                    app.current_play_tracks = TrackTable {
                        tracks: album_detail.tracks.clone(),
                        selected_index: album_detail.selected_track_index,
                        context: Some(TrackTableContext::AlbumDetail),
                    };
                    // 将下一曲播放队列置为空
                    app.next_play_tracks = vec![];
                }
            }
        }
        Key::Char('s') => {
            let (selected_index, tracks) = (
                app.album_detail.as_ref().unwrap().selected_track_index,
                &app.album_detail.as_ref().unwrap().tracks,
            );
            if let Some(track) = tracks.get(selected_index) {
                let id = track.id;
                app.dispatch(IoEvent::ToggleLikeTrack(id));
            };
        }
        // 加入下一曲播放队列
        k if k == app.user_config.keys.add_item_to_queue => {
            let (selected_index, tracks) = (
                app.album_detail.as_ref().unwrap().selected_track_index,
                &app.album_detail.as_ref().unwrap().tracks,
            );
            if let Some(track) = tracks.get(selected_index) {
                let track = track.clone();
                app.dispatch(IoEvent::AddToQueue(track));
            };
        }
        _ => {}
    }
}

fn handle_high_event(app: &mut App) {
    if let Some(album_detail) = &mut app.album_detail {
        let next_index = common_key_events::on_high_press_handler();
        album_detail.selected_track_index = next_index;
    }
}

fn handle_middle_event(app: &mut App) {
    if let Some(album_detail) = &mut app.album_detail {
        let next_index = common_key_events::on_middle_press_handler(&album_detail.tracks);
        album_detail.selected_track_index = next_index;
    }
}

fn handle_low_event(app: &mut App) {
    if let Some(album_detail) = &mut app.album_detail {
        let next_index = common_key_events::on_low_press_handler(&album_detail.tracks);
        album_detail.selected_track_index = next_index;
    }
}
