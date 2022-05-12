use super::common_key_events;
use crate::event::Key;
use crate::model::artist::ArtistBlock;
use crate::{App, IoEvent};

pub fn handler(key: Key, app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match key {
            Key::Esc => {
                artist.artist_detail_selected_block = ArtistBlock::Empty;
            }
            k if common_key_events::down_event(k) => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_down_press_on_selected_block(app);
                } else {
                    handle_down_press_on_hovered_block(app);
                }
            }
            k if common_key_events::up_event(k) => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_up_press_on_selected_block(app);
                } else {
                    handle_up_press_on_hovered_block(app);
                }
            }
            k if common_key_events::left_event(k) => {
                artist.artist_detail_selected_block = ArtistBlock::Empty;
                match artist.artist_detail_hovered_block {
                    ArtistBlock::Tracks => common_key_events::handle_left_event(app),
                    ArtistBlock::Albums => {
                        artist.artist_detail_hovered_block = ArtistBlock::Tracks;
                    }
                    ArtistBlock::SimiArtists => {
                        artist.artist_detail_hovered_block = ArtistBlock::Albums;
                    }
                    ArtistBlock::Empty => {}
                }
            }
            k if common_key_events::right_event(k) => {
                artist.artist_detail_selected_block = ArtistBlock::Empty;
                handle_down_press_on_hovered_block(app);
            }
            k if common_key_events::high_event(k) => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_high_press_on_selected_block(app);
                }
            }
            k if common_key_events::middle_event(k) => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_middle_press_on_selected_block(app);
                }
            }
            k if common_key_events::low_event(k) => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_low_press_on_selected_block(app);
                }
            }
            Key::Enter => {
                if artist.artist_detail_selected_block != ArtistBlock::Empty {
                    handle_enter_event_on_selected_block(app);
                } else {
                    handle_enter_event_on_hovered_block(app);
                }
            }
            Key::Char('s') => {
                if app
                    .artist_detail
                    .as_ref()
                    .unwrap()
                    .artist_detail_selected_block
                    == ArtistBlock::Tracks
                {
                    handle_toggle_like_event(app);
                }
            }
            _ => {}
        };
    }
}

fn handle_toggle_like_event(app: &mut App) {
    let (selected_index, tracks) = (
        app.artist_detail.as_ref().unwrap().selected_track_index,
        &app.artist_detail.as_ref().unwrap().tracks,
    );
    if let Some(track) = tracks.get(selected_index) {
        let id = track.id;
        app.dispatch(IoEvent::ToggleLikeTrack(id));
    };
}

fn handle_down_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.tracks,
                    Some(artist.selected_track_index),
                );
                artist.selected_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.albums,
                    Some(artist.selected_album_index),
                );
                artist.selected_album_index = next_index;
            }
            ArtistBlock::SimiArtists => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.simi_artists,
                    Some(artist.selected_simi_artist_index),
                );
                artist.selected_simi_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_down_press_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_hovered_block {
            ArtistBlock::Tracks => {
                artist.artist_detail_hovered_block = ArtistBlock::Albums;
            }
            ArtistBlock::Albums => {
                artist.artist_detail_hovered_block = ArtistBlock::SimiArtists;
            }
            ArtistBlock::SimiArtists => {
                artist.artist_detail_hovered_block = ArtistBlock::Tracks;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_up_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.tracks,
                    Some(artist.selected_track_index),
                );
                artist.selected_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.albums,
                    Some(artist.selected_album_index),
                );
                artist.selected_album_index = next_index;
            }
            ArtistBlock::SimiArtists => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.simi_artists,
                    Some(artist.selected_simi_artist_index),
                );
                artist.selected_simi_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_up_press_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_hovered_block {
            ArtistBlock::Tracks => {
                artist.artist_detail_hovered_block = ArtistBlock::SimiArtists;
            }
            ArtistBlock::Albums => {
                artist.artist_detail_hovered_block = ArtistBlock::Tracks;
            }
            ArtistBlock::SimiArtists => {
                artist.artist_detail_hovered_block = ArtistBlock::Albums;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_high_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let next_index = common_key_events::on_high_press_handler();
                artist.selected_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_high_press_handler();
                artist.selected_album_index = next_index;
            }
            ArtistBlock::SimiArtists => {
                let next_index = common_key_events::on_high_press_handler();
                artist.selected_simi_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_middle_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let next_index = common_key_events::on_middle_press_handler(&artist.tracks);
                artist.selected_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_middle_press_handler(&artist.albums);
                artist.selected_album_index = next_index;
            }
            ArtistBlock::SimiArtists => {
                let next_index = common_key_events::on_middle_press_handler(&artist.simi_artists);
                artist.selected_simi_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_low_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let next_index = common_key_events::on_low_press_handler(&artist.tracks);
                artist.selected_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_low_press_handler(&artist.albums);
                artist.selected_album_index = next_index;
            }
            ArtistBlock::SimiArtists => {
                let next_index = common_key_events::on_low_press_handler(&artist.simi_artists);
                artist.selected_simi_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_enter_event_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail.clone() {
        match artist.artist_detail_selected_block {
            ArtistBlock::Tracks => {
                let selected_index = artist.selected_track_index;
                if let Some(track) = artist.tracks.get(selected_index) {
                    app.dispatch(IoEvent::StartPlayback(track.clone()));
                }
            }
            ArtistBlock::Albums => {}
            ArtistBlock::SimiArtists => {
                let selected_index = artist.selected_simi_artist_index;
                let artist_id = artist.simi_artists[selected_index].id.clone();
                let artist_name = artist.simi_artists[selected_index].name.clone();
                app.dispatch(IoEvent::GetArtistDetail(artist_id, artist_name.unwrap()));
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_enter_event_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist_detail {
        match artist.artist_detail_hovered_block {
            ArtistBlock::Tracks => artist.artist_detail_selected_block = ArtistBlock::Tracks,
            ArtistBlock::Albums => artist.artist_detail_selected_block = ArtistBlock::Albums,
            ArtistBlock::SimiArtists => {
                artist.artist_detail_selected_block = ArtistBlock::SimiArtists
            }
            ArtistBlock::Empty => {}
        }
    }
}
