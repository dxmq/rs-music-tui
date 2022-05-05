use crate::app::App;
use crate::event::{IoEvent, Key};
use crate::handlers::common_key_events;
use crate::handlers::search::SearchResultBlock;
use crate::model::context::TrackTableContext;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        k if common_key_events::down_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_down_press_on_selected_block(app);
            } else {
                handle_down_press_on_hovered_block(app);
            }
        }
        k if common_key_events::up_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_up_press_on_selected_block(app);
            } else {
                handle_up_press_on_hovered_block(app);
            }
        }
        k if common_key_events::left_event(k) => {
            app.search_results.selected_block = SearchResultBlock::Empty;
            match app.search_results.hovered_block {
                SearchResultBlock::AlbumSearch => {
                    common_key_events::handle_left_event(app);
                }
                SearchResultBlock::TrackSearch => {
                    common_key_events::handle_left_event(app);
                }
                SearchResultBlock::ArtistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::TrackSearch;
                }
                SearchResultBlock::PlaylistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
                }
                SearchResultBlock::Empty => {}
            }
        }
        k if common_key_events::right_event(k) => {
            app.search_results.selected_block = SearchResultBlock::Empty;
            match app.search_results.hovered_block {
                SearchResultBlock::AlbumSearch => {
                    app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
                }
                SearchResultBlock::TrackSearch => {
                    app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
                }
                SearchResultBlock::ArtistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::TrackSearch;
                }
                SearchResultBlock::PlaylistSearch => {
                    app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
                }
                SearchResultBlock::Empty => {}
            }
        }
        k if common_key_events::high_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_high_press_on_selected_block(app);
            }
        }
        k if common_key_events::middle_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_middle_press_on_selected_block(app);
            }
        }
        k if common_key_events::low_event(k) => {
            if app.search_results.selected_block != SearchResultBlock::Empty {
                handle_low_press_on_selected_block(app)
            }
        }
        Key::Char('s') => match app.search_results.selected_block {
            SearchResultBlock::TrackSearch => {
                handle_toggle_like_event(app);
            }
            SearchResultBlock::PlaylistSearch => {
                handle_toggle_subscribe_playlist_event(app);
            }
            SearchResultBlock::Empty => {}
            _ => {}
        },
        Key::Enter => match app.search_results.selected_block {
            SearchResultBlock::Empty => handle_enter_event_on_hovered_block(app),
            SearchResultBlock::PlaylistSearch => {
                app.playlist_offset = 0;
                handle_enter_event_on_selected_block(app);
            }
            _ => handle_enter_event_on_selected_block(app),
        },
        _ => {}
    }
}

fn handle_toggle_subscribe_playlist_event(app: &mut App) {
    let playlists = app.search_results.playlists.clone().unwrap();
    let selected_index = app.search_results.selected_playlists_index.unwrap();
    if let Some(playlist) = playlists.get(selected_index) {
        app.dispatch(IoEvent::ToggleSubscribePlaylist(playlist.id));
    };
}

fn handle_toggle_like_event(app: &mut App) {
    let tracks = app.search_results.tracks.clone().unwrap();
    let selected_index = app.search_results.selected_tracks_index.unwrap();
    if let Some(track) = tracks.get(selected_index) {
        let id = track.id;
        app.dispatch(IoEvent::ToggleLikeTrack(id));
    };
}

fn handle_down_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_down_press_handler(
                    result,
                    app.search_results.selected_album_index,
                );
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::TrackSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_down_press_handler(
                    result,
                    app.search_results.selected_tracks_index,
                );
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_down_press_handler(
                    result,
                    app.search_results.selected_artists_index,
                );
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_down_press_handler(
                    result,
                    app.search_results.selected_playlists_index,
                );
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_down_press_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::TrackSearch => {
            app.search_results.hovered_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::ArtistSearch => {
            app.search_results.hovered_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::Empty => {}
        _ => {}
    }
}

fn handle_up_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_up_press_handler(
                    result,
                    app.search_results.selected_album_index,
                );
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::TrackSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_up_press_handler(
                    result,
                    app.search_results.selected_tracks_index,
                );
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_up_press_handler(
                    result,
                    app.search_results.selected_artists_index,
                );
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_up_press_handler(
                    result,
                    app.search_results.selected_playlists_index,
                );
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_up_press_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
            app.search_results.hovered_block = SearchResultBlock::TrackSearch;
        }
        SearchResultBlock::PlaylistSearch => {
            app.search_results.hovered_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::Empty => {}
        _ => {}
    }
}

fn handle_high_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(_result) = &app.search_results.albums {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::TrackSearch => {
            if let Some(_result) = &app.search_results.tracks {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(_result) = &app.search_results.artists {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(_result) = &app.search_results.playlists {
                let next_index = common_key_events::on_high_press_handler();
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_middle_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_middle_press_handler(result);
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::TrackSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_middle_press_handler(result);
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_middle_press_handler(result);
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_middle_press_handler(result);
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_low_press_on_selected_block(app: &mut App) {
    match app.search_results.selected_block {
        SearchResultBlock::AlbumSearch => {
            if let Some(result) = &app.search_results.albums {
                let next_index = common_key_events::on_low_press_handler(result);
                app.search_results.selected_album_index = Some(next_index);
            }
        }
        SearchResultBlock::TrackSearch => {
            if let Some(result) = &app.search_results.tracks {
                let next_index = common_key_events::on_low_press_handler(result);
                app.search_results.selected_tracks_index = Some(next_index);
            }
        }
        SearchResultBlock::ArtistSearch => {
            if let Some(result) = &app.search_results.artists {
                let next_index = common_key_events::on_low_press_handler(result);
                app.search_results.selected_artists_index = Some(next_index);
            }
        }
        SearchResultBlock::PlaylistSearch => {
            if let Some(result) = &app.search_results.playlists {
                let next_index = common_key_events::on_low_press_handler(result);
                app.search_results.selected_playlists_index = Some(next_index);
            }
        }
        SearchResultBlock::Empty => {}
    }
}

fn handle_enter_event_on_selected_block(app: &mut App) {
    match &app.search_results.selected_block {
        // SearchResultBlock::AlbumSearch => {
        //     if let (Some(index), Some(albums_result)) = (
        //         &app.search_results.selected_album_index,
        //         &app.search_results.albums,
        //     ) {
        //         if let Some(album) = albums_result.items.get(index.to_owned()).cloned() {
        //             app.track_table.context = Some(TrackTableContext::AlbumSearch);
        //             app.dispatch(IoEvent::GetAlbumTracks(Box::new(album)));
        //         };
        //     }
        // }
        SearchResultBlock::TrackSearch => {
            let index = app.search_results.selected_tracks_index;
            let tracks = app.search_results.tracks.clone();
            if let Some(tracks) = tracks {
                if let Some(track) = tracks.get(index.unwrap()) {
                    app.dispatch(IoEvent::StartPlayback(track.clone()));
                }
            }
        }
        // SearchResultBlock::ArtistSearch => {
        //     if let Some(index) = &app.search_results.selected_artists_index {
        //         if let Some(result) = app.search_results.artists.clone() {
        //             if let Some(artist) = result.items.get(index.to_owned()) {
        //                 app.get_artist(artist.id.clone(), artist.name.clone());
        //                 app.push_navigation_stack(RouteId::Artist, ActiveBlock::ArtistBlock);
        //             };
        //         };
        //     };
        // }
        SearchResultBlock::PlaylistSearch => {
            if let (Some(index), Some(playlists_result)) = (
                app.search_results.selected_playlists_index,
                &app.search_results.playlists,
            ) {
                if let Some(playlist) = playlists_result.get(index) {
                    // Go to playlist tracks table
                    app.track_table.context = Some(TrackTableContext::PlaylistSearch);
                    let playlist_id = playlist.id.to_owned();
                    app.dispatch(IoEvent::GetPlaylistTracks(playlist_id));
                };
            }
        }
        SearchResultBlock::Empty => {}
        _ => {}
    };
}

fn handle_enter_event_on_hovered_block(app: &mut App) {
    match app.search_results.hovered_block {
        SearchResultBlock::AlbumSearch => {
            let next_index = app.search_results.selected_album_index.unwrap_or(0);

            app.search_results.selected_album_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::AlbumSearch;
        }
        SearchResultBlock::TrackSearch => {
            let next_index = app.search_results.selected_tracks_index.unwrap_or(0);

            app.search_results.selected_tracks_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::TrackSearch;
        }
        SearchResultBlock::ArtistSearch => {
            let next_index = app.search_results.selected_artists_index.unwrap_or(0);

            app.search_results.selected_artists_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::ArtistSearch;
        }
        SearchResultBlock::PlaylistSearch => {
            let next_index = app.search_results.selected_playlists_index.unwrap_or(0);

            app.search_results.selected_playlists_index = Some(next_index);
            app.search_results.selected_block = SearchResultBlock::PlaylistSearch;
        }
        SearchResultBlock::Empty => {}
    };
}
