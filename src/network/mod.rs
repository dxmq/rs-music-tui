pub(crate) mod cloud_music;

use std::sync::Arc;
use std::time::Instant;

use anyhow::Error;
use tokio::sync::Mutex;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::IoEvent;
use crate::model::context::CurrentlyPlaybackContext;
use crate::model::enums::{CurrentlyPlayingType, PlayingItem, RepeatState};
use crate::model::track::Track;
use crate::network::cloud_music::CloudMusic;
use crate::player::Nplayer;

pub struct Network<'a> {
    // 最大搜索限制
    large_search_limit: u32,
    // 最小搜索限制
    small_search_limit: u32,
    pub app: &'a Arc<Mutex<App>>,
    pub player: Nplayer,
    pub cloud_music: CloudMusic,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Network {
            large_search_limit: 20,
            small_search_limit: 4,
            app,
            cloud_music: CloudMusic::default(),
            player: Nplayer::new(),
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            // IoEvent::GetSearchResults(search_term) => {}
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
            IoEvent::GetPlaylistTracks(playlist_id, playlist_offset) => {
                self.load_playlist_tracks(playlist_id, playlist_offset)
                    .await;
            }
            IoEvent::StartPlayback(track) => {
                self.start_playback(track).await;
            }
            IoEvent::TogglePlayBack => {
                self.toggle_playback().await;
            }
            _ => {}
        }

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn toggle_playback(&mut self) {
        let mut app = self.app.lock().await;
        let context = app.current_playback_context.clone();
        match context {
            Some(mut context) => {
                if self.player.is_playing() {
                    context.is_playing = false;
                    context.progress_ms = Some(app.song_progress_ms as u32);
                    app.instant_since_last_current_playback_poll = Instant::now();
                    app.current_playback_context = Some(context);
                    self.player.pause();
                } else {
                    context.is_playing = true;
                    context.progress_ms = Some(app.song_progress_ms as u32);
                    app.instant_since_last_current_playback_poll = Instant::now();
                    app.current_playback_context = Some(context);
                    self.player.play();
                }
            }
            None => {
                self.player.pause();
            }
        }
    }

    async fn start_playback(&mut self, track: Track) {
        match self.cloud_music.song_url(vec![track.id]).await {
            Ok(urls) => {
                if let Some(track_url) = urls.get(0) {
                    let mut app = self.app.lock().await;
                    let context = CurrentlyPlaybackContext {
                        is_playing: true,
                        progress_ms: Some(0),
                        timestamp: 0,
                        currently_playing_type: CurrentlyPlayingType::Track,
                        repeat_state: RepeatState::Off,
                        shuffle_state: false,
                        item: Some(PlayingItem::Track(track)),
                    };

                    app.instant_since_last_current_playback_poll = Instant::now();
                    self.player.play_url(track_url.url.as_str());

                    app.current_playback_context = Some(context);
                }
            }
            Err(e) => self.handle_error(e).await,
        }

        let mut app = self.app.lock().await;
        app.seek_ms.take();
        app.is_fetching_current_playback = false;
    }

    async fn load_playlist_tracks(&mut self, playlist_idk: usize, playlist_offset: u32) {
        println!("{}", playlist_offset);
        match self.cloud_music.playlist_tracks(playlist_idk).await {
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
            .cloud_music
            .current_user_playlists(self.large_search_limit, None, self.app)
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
        match self.cloud_music.current_user().await {
            Ok(user) => {
                let mut app = self.app.lock().await;
                app.user = user
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    pub async fn handle_error(&mut self, e: Error) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }
}

#[tokio::main]
pub async fn start_tokio(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
