use crate::app::{ActiveBlock, App, LIBRARY_OPTIONS, RouteId};
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::model::context::TrackTableContext;

pub fn handles(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &LIBRARY_OPTIONS,
                Some(app.library.selected_index),
            );
            app.library.selected_index = next_index;
        }
        k if common_key_events::high_event(k) => {
            let next_index = common_key_events::on_high_press_handler();
            app.library.selected_index = next_index;
        }
        k if common_key_events::middle_event(k) => {
            let next_index = common_key_events::on_middle_press_handler(&LIBRARY_OPTIONS);
            app.library.selected_index = next_index;
        }
        k if common_key_events::low_event(k) => {
            let next_index = common_key_events::on_low_press_handler(&LIBRARY_OPTIONS);
            app.library.selected_index = next_index
        }
        Key::Enter => {
            if app.library.selected_index == 0 {
                let playlist_id = app.my_like_playlist_id;
                app.dispatch(IoEvent::GetPlaylistTracks(playlist_id));
            } else if app.library.selected_index == 1 {
                app.track_table.context = Some(TrackTableContext::RecentlyPlayed);
                app.dispatch(IoEvent::GetRecentlyPlayed);
            } else if app.library.selected_index == 2 {
                app.track_table.context = Some(TrackTableContext::RecommendedTracks);
                app.dispatch(IoEvent::GetRecommendTracks);
            } else if app.library.selected_index == 3 {
                app.dispatch(IoEvent::GetArtistSubList);
                app.push_navigation_stack(RouteId::Artists, ActiveBlock::Artists);
            }
        }
        _ => {}
    }
}
