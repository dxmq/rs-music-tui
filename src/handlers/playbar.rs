use super::{
    super::app::{ActiveBlock, App},
    common_key_events,
};
use crate::event::{IoEvent, Key};
use crate::model::context::CurrentlyPlaybackContext;
use crate::model::enums::PlayingItem;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::up_event(k) => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::MyPlaylists));
        }
        k if Key::Enter == k => app.dispatch(IoEvent::TogglePlayBack),
        // Key::Char('s') => {
        //     if let Some(CurrentlyPlaybackContext {
        //                     item: Some(item), ..
        //                 }) = app.current_playback_context.to_owned()
        //     {
        //         match item {
        //             PlayingItem::Track(track) => {
        //                 if let Some(track_id) = track.id {
        //                     app.dispatch(IoEvent::ToggleSaveTrack(track_id));
        //                 }
        //             }
        //             PlayingItem::Episode(episode) => {
        //                 app.dispatch(IoEvent::ToggleSaveTrack(episode.id));
        //             }
        //         };
        //     };
        // }
        _ => {}
    };
}
