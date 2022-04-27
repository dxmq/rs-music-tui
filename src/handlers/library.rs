use crate::app::{ActiveBlock, App, RouteId, LIBRARY_OPTIONS};
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
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Enter => {
            if app.library.selected_index == 0 {
                app.dispatch(IoEvent::GetRecentlyPlayed(500));
                app.push_navigation_stack(RouteId::RecentlyPlayed, ActiveBlock::RecentlyPlayed);
            } else if app.library.selected_index == 1 {
                app.track_table.context = Some(TrackTableContext::MyPlaylists);
                app.dispatch(IoEvent::GetRecommendTracks);
            }
        }
        _ => {}
    }
}
