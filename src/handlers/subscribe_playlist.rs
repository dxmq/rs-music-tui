use crate::app::{ActiveBlock, App, RouteId};
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::model::context::{DialogContext, TrackTableContext};
use crate::model::dialog::Dialog;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            match &app.sub_playlists {
                Some(p) => {
                    if let Some(selected_sub_playlist_index) = app.selected_sub_playlist_index {
                        let next_index = common_key_events::on_down_press_handler(
                            p,
                            Some(selected_sub_playlist_index),
                        );
                        app.selected_sub_playlist_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_key_events::up_event(k) => {
            match &app.sub_playlists {
                Some(p) => {
                    let next_index =
                        common_key_events::on_up_press_handler(p, app.selected_sub_playlist_index);
                    app.selected_sub_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::high_event(k) => {
            match &app.sub_playlists {
                Some(_p) => {
                    let next_index = common_key_events::on_high_press_handler();
                    app.selected_sub_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::middle_event(k) => {
            match &app.sub_playlists {
                Some(p) => {
                    let next_index = common_key_events::on_middle_press_handler(p);
                    app.selected_sub_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        k if common_key_events::low_event(k) => {
            match &app.sub_playlists {
                Some(p) => {
                    let next_index = common_key_events::on_low_press_handler(p);
                    app.selected_sub_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        Key::Enter => {
            if let (Some(sub_playlists), Some(selected_sub_playlist_index)) =
                (&app.sub_playlists, &app.selected_sub_playlist_index)
            {
                app.active_sub_playlist_index = Some(selected_sub_playlist_index.to_owned());
                app.track_table.context = Some(TrackTableContext::MyPlaylists);
                // app.playlist_offset = 0;
                if let Some(selected_playlist) =
                    sub_playlists.get(selected_sub_playlist_index.to_owned())
                {
                    let selected_playlist = selected_playlist.id.to_owned();
                    app.dispatch(IoEvent::GetPlaylistTracks(selected_playlist));
                }
            };
        }
        Key::Char('D') => {
            if let (Some(sub_playlists), Some(selected_index)) =
                (&app.sub_playlists, app.selected_sub_playlist_index)
            {
                let selected_playlist = &sub_playlists[selected_index].name;
                app.dialog = Some(Dialog {
                    tips: "确定要取消收藏歌单：".to_string(),
                    item_name: selected_playlist.clone(),
                    confirm: false
                });
                app.push_navigation_stack(
                    RouteId::Dialog,
                    ActiveBlock::Dialog(DialogContext::SubPlaylist),
                );
            }
        }
        _ => {}
    }
}
