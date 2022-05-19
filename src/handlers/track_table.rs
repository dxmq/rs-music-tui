use crate::app::{ActiveBlock, App, RouteId};
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::handlers::common_key_events::KeyAction;
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
        k if k == app.user_config.keys.next_page => {
            let next_index = common_key_events::on_down_or_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
                KeyAction::Down,
                20,
            );
            app.track_table.selected_index = next_index;
        }
        k if k == app.user_config.keys.previous_page => {
            let next_index = common_key_events::on_down_or_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
                KeyAction::Up,
                20,
            );
            app.track_table.selected_index = next_index;
        }
        k if k == app.user_config.keys.jump_to_end => {
            app.track_table.selected_index = 0;
        }
        k if k == app.user_config.keys.jump_to_start => {
            app.track_table.selected_index = app.track_table.tracks.len() - 1;
        }
        // 跳转到歌手详情页
        k if k == app.user_config.keys.jump_to_artist_detail => {
            let (selected_index, tracks) =
                (&app.track_table.selected_index, &app.track_table.tracks);
            if let Some(track) = tracks.get(*selected_index) {
                if track.artists.len() == 1 {
                    let artist = track.artists.get(0).unwrap();
                    let artist_name = artist.name.clone().unwrap_or_else(|| "".to_string());
                    let id = artist.id;
                    app.dispatch(IoEvent::GetArtistDetail(id, artist_name));
                    app.push_navigation_stack(RouteId::ArtistDetail, ActiveBlock::ArtistDetail);
                }
            };
        }
        // 跳转到歌手专辑页
        k if k == app.user_config.keys.jump_to_artist_album => {
            let (selected_index, tracks) =
                (&app.track_table.selected_index, &app.track_table.tracks);
            if let Some(track) = tracks.get(*selected_index) {
                let album = track.album.clone();
                app.track_table.context = Some(TrackTableContext::AlbumDetail);
                app.dispatch(IoEvent::GetAlbumTracks(Box::new(album)));
            };
        }
        k if k == Key::Char('s') => handle_toggle_like_event(app),
        // 加入下一曲播放队列
        k if k == app.user_config.keys.add_item_to_queue => {
            let (selected_index, tracks) =
                (&app.track_table.selected_index, &app.track_table.tracks);
            if let Some(track) = tracks.get(*selected_index) {
                app.next_play_tracks.push(track.clone())
            };
        }
        Key::Enter => {
            on_enter(app);
        }
        _ => {}
    }
}

fn handle_toggle_like_event(app: &mut App) {
    let (selected_index, tracks) = (&app.track_table.selected_index, &app.track_table.tracks);
    if let Some(track) = tracks.get(*selected_index) {
        let id = track.id;
        app.dispatch(IoEvent::ToggleLikeTrack(id));
    };
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
            TrackTableContext::RecentlyPlayed => {
                if track.is_some() {
                    let tracks = tracks.clone();
                    app.my_play_tracks = TrackTable {
                        tracks,
                        selected_index,
                        context: Some(TrackTableContext::RecentlyPlayed),
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
