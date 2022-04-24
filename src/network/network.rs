use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{anyhow, Error};
use ncmapi::types::{Artist, Playlist, PlaylistDetail, Song, SongUrl, UserPlaylistResp};
use tokio::sync::Mutex;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::IoEvent;
use crate::model::context::{Action, CurrentlyPlaybackContext};
use crate::model::device::Device;
use crate::model::enums::{CurrentlyPlayingType, DisallowKey, PlayingItem, RepeatState};
use crate::network::api;
use crate::network::ncm::{CloudMusic, TError, TResult};
use crate::player::Nplayer;

pub struct Network<'a> {
    // 最大搜索限制
    large_search_limit: u32,
    // 最小搜索限制
    small_search_limit: u32,
    pub app: &'a Arc<Mutex<App>>,
    pub player: Nplayer,
    pub ncm: CloudMusic,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Network {
            large_search_limit: 20,
            small_search_limit: 4,
            app,
            ncm: Default::default(),
            player: Nplayer::new(),
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetSearchResults(search_term) => {}
            IoEvent::UpdateSearchLimits(large_search_limit, small_search_limit) => {
                self.large_search_limit = large_search_limit;
                self.small_search_limit = small_search_limit;
            }
            IoEvent::GetPlaylists => {
                self.load_current_user_playlists().await;
            }
            IoEvent::GetUser => {
                self.load_user().await;
            }
            IoEvent::GetPlaylistTracks(playlist_id) => {
                self.load_playlist_tracks(playlist_id).await;
            }
            IoEvent::StartPlayback(song) => {
                self.start_playback(song).await;
            }
            // IoEvent::CurrentUserSavedTracksContains(track_ids) => {
            //     self.current_user_saved_tracks_contains(track_ids).await;
            // }
            _ => {}
        }

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }
    //
    // async fn current_user_saved_tracks_contains(&mut self, ids: Vec<String>) {
    //     match self.spotify.current_user_saved_tracks_contains(&ids).await {
    //         Ok(is_saved_vec) => {
    //             let mut app = self.app.lock().await;
    //             for (i, id) in ids.iter().enumerate() {
    //                 if let Some(is_liked) = is_saved_vec.get(i) {
    //                     if *is_liked {
    //                         app.liked_song_ids_set.insert(id.to_string());
    //                     } else {
    //                         // The song is not liked, so check if it should be removed
    //                         if app.liked_song_ids_set.contains(id) {
    //                             app.liked_song_ids_set.remove(id);
    //                         }
    //                     }
    //                 };
    //             }
    //         }
    //         Err(e) => {
    //             self.handle_error(anyhow!(e)).await;
    //         }
    //     }
    // }

    async fn start_playback(&mut self, song: Song) {
        match self.ncm.song_url(vec![song.id]).await {
            Ok(urls) => {
                if let Some(song_url) = urls.get(0) {
                    let mut app = self.app.lock().await;
                    let disallows: HashMap<DisallowKey, bool> = HashMap::new();
                    self.player.play_url(song_url.url.as_str());
                    let context = CurrentlyPlaybackContext {
                        is_playing: true,
                        progress_ms: Some(0),
                        timestamp: 0,
                        context: None,
                        currently_playing_type: CurrentlyPlayingType::Track,
                        active: Action { disallows },
                        repeat_state: RepeatState::Off,
                        shuffle_state: false,
                        item: Some(PlayingItem::Track(song)),
                    };
                    app.instant_since_last_current_playback_poll = Instant::now();
                    app.current_playback_context = Some(context);
                }
            }
            Err(e) => self.handle_error(e).await,
        }

        let mut app = self.app.lock().await;
        app.seek_ms.take();
        app.is_fetching_current_playback = false;
    }

    async fn set_playlist_tracks_to_table(&mut self, playlist_track_page: &PlaylistDetail) {
        self.set_tracks_to_table(playlist_track_page.tracks.clone())
            .await;
    }

    async fn set_tracks_to_table(&mut self, tracks: Vec<Song>) {
        let mut app = self.app.lock().await;
        app.track_table.tracks = tracks.clone();

        // Send this event round (don't block here)
        app.dispatch(IoEvent::CurrentUserSavedTracksContains(
            tracks
                .into_iter()
                .filter_map(|item| Option::from(item.id.to_string()))
                .collect::<Vec<String>>(),
        ));
    }

    async fn load_playlist_tracks(&mut self, playlist_idk: usize) {
        match self.ncm.playlist_tracks(playlist_idk).await {
            Ok(playlist_tracks) => {
                let mut app = self.app.lock().await;

                app.track_table.tracks = playlist_tracks.tracks.clone();
                // self.set_playlist_tracks_to_table(&playlist_tracks).await;
                app.playlist_tracks = Some(playlist_tracks);
                app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn load_current_user_playlists(&mut self) {
        let result = self
            .ncm
            .current_user_playlists(self.large_search_limit, None, &self.app)
            .await;
        match result {
            Ok(list) => {
                let mut app = self.app.lock().await;
                app.playlists = Some(list);
                app.selected_playlist_index = Some(0);
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn load_user(&mut self) {
        match self.ncm.current_user().await {
            Ok(user) => {
                let mut app = self.app.lock().await;
                app.user = user
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    pub async fn handle_error(&mut self, e: TError) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }
}

#[tokio::main]
pub async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
