use std::sync::Arc;
use std::time::Instant;

use anyhow::Error;
use tokio::sync::Mutex;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::IoEvent;
use crate::model::context::{CurrentlyPlaybackContext, TrackTableContext};
use crate::model::enums::{CurrentlyPlayingType, PlayingItem, RepeatState};
use crate::model::table::TrackTable;
use crate::model::track::Track;
use crate::network::cloud_music::CloudMusic;
use crate::player::Nplayer;

pub(crate) mod cloud_music;

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
            IoEvent::GetPlaylistTracks(playlist_id) => {
                self.load_playlist_tracks(playlist_id).await;
            }
            IoEvent::StartPlayback(track) => {
                self.start_playback(track).await;
            }
            IoEvent::TogglePlayBack => {
                self.toggle_playback().await;
            }
            IoEvent::GetRecentlyPlayed(usize) => {
                self.load_recently_played(usize).await;
            }
            IoEvent::GetRecommendTracks => {
                self.load_recommend_tracks().await;
            }
            IoEvent::DecreaseVolume => {
                self.decrease_volume().await;
            }
            IoEvent::IncreaseVolume => {
                self.increase_volume().await;
            }
            IoEvent::GetLikeList => {
                self.load_like_track_id_list().await;
            }
            IoEvent::GetLyric(track_id) => {
                self.load_track_lyric(track_id).await;
            }
            _ => {}
        }

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn load_track_lyric(&mut self, track_id: usize) {
        let mut app = self.app.lock().await;
        let lyric = self.cloud_music.lyric(track_id).await;
        match lyric {
            Ok(lyric) => {
                app.lyric_index = 0;
                app.lyric = Some(lyric);
            }
            Err(_) => {
                app.lyric_index = 0;
                app.lyric = None;
            }
        }
        app.push_navigation_stack(RouteId::Lyric, ActiveBlock::Lyric);
    }

    async fn load_like_track_id_list(&mut self) {
        let mut app = self.app.lock().await;
        if let Some(profile) = app.user.clone() {
            if let Ok(liked_track_ids) = self.cloud_music.like_track_id_list(profile.user_id).await
            {
                app.liked_track_ids_set = liked_track_ids;
            }
        }
    }

    async fn decrease_volume(&mut self) {
        self.player.decrease_volume();
        let mut app = self.app.lock().await;
        app.volume = self.player.get_volume();
    }

    async fn increase_volume(&mut self) {
        self.player.increase_volume();
        let mut app = self.app.lock().await;
        app.volume = self.player.get_volume();
    }

    async fn load_recommend_tracks(&mut self) {
        match self.cloud_music.recommend_song_list().await {
            Ok(tracks) => {
                let mut app = self.app.lock().await;
                app.track_table = TrackTable {
                    tracks,
                    selected_index: 0,
                    context: Some(TrackTableContext::RecommendedTracks),
                };
                app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
                app.title = String::from("每日推荐");
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn load_recently_played(&mut self, limit: u32) {
        match self.cloud_music.recent_song_list(500).await {
            Ok(recent_play_list) => {
                let mut app = self.app.lock().await;

                if limit == 500 {
                    app.track_table.tracks = recent_play_list;
                    app.track_table.context = Some(TrackTableContext::RecentlyPlayed);
                    app.title = "最近播放".to_string();
                    app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
                } else if limit == 1 {
                    let play_list = recent_play_list.clone();
                    let track = recent_play_list.get(0).unwrap();
                    let context = CurrentlyPlaybackContext {
                        is_playing: false,
                        progress_ms: Some(0),
                        timestamp: 0,
                        currently_playing_type: CurrentlyPlayingType::Track,
                        repeat_state: RepeatState::Off,
                        item: Some(PlayingItem::Track(track.clone())),
                    };
                    app.current_playback_context = Some(context);
                    app.my_play_tracks = TrackTable {
                        tracks: play_list,
                        selected_index: 0,
                        context: Some(TrackTableContext::RecentlyPlayed),
                    };
                }
                app.volume = self.player.get_volume();
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
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
                    match self.player.get_duration() {
                        Some(_) => {
                            context.is_playing = true;
                            context.progress_ms = Some(app.song_progress_ms as u32);
                            app.instant_since_last_current_playback_poll = Instant::now();
                            app.current_playback_context = Some(context);
                            self.player.play();
                        }
                        None => {
                            let track = context.item.as_ref().unwrap();
                            let PlayingItem::Track(track) = track;
                            match self.cloud_music.song_url(vec![track.id]).await {
                                Ok(urls) => {
                                    match self.player.play_url(urls.get(0).unwrap().url.as_str()) {
                                        Ok(()) => {
                                            context.is_playing = true;
                                            context.progress_ms = Some(app.song_progress_ms as u32);
                                            app.instant_since_last_current_playback_poll =
                                                Instant::now();
                                            app.current_playback_context = Some(context);
                                        }
                                        Err(e) => {
                                            app.handle_error(e);
                                        }
                                    }
                                }
                                Err(e) => self.handle_error(e).await,
                            }
                        }
                    }
                }
                app.volume = self.player.get_volume();
            }
            None => {
                self.player.pause();
            }
        }
    }

    async fn start_playback(&mut self, mut track: Track) {
        match self.cloud_music.song_url(vec![track.id]).await {
            Ok(urls) => {
                if let Some(track_url) = urls.get(0) {
                    let mut app = self.app.lock().await;
                    match self.player.play_url(track_url.url.as_str()) {
                        Ok(()) => {
                            match app.current_playback_context.clone() {
                                Some(mut context) => {
                                    context.is_playing = true;
                                    if track_url.fee == 1 {
                                        if let Some(info) = track_url.free_trial_info.clone() {
                                            let duration = (info.end - info.start) * 1000;
                                            track.duration = duration;
                                        }
                                    }
                                    context.item = Some(PlayingItem::Track(track));
                                    app.current_playback_context = Some(context);
                                }
                                None => {
                                    let context = CurrentlyPlaybackContext {
                                        is_playing: true,
                                        progress_ms: Some(0),
                                        timestamp: 0,
                                        currently_playing_type: CurrentlyPlayingType::Track,
                                        repeat_state: RepeatState::Off,
                                        item: Some(PlayingItem::Track(track)),
                                    };
                                    app.current_playback_context = Some(context);
                                }
                            }
                            app.instant_since_last_current_playback_poll = Instant::now();

                            app.volume = self.player.get_volume();
                        }
                        Err(e) => {
                            app.handle_error(e);
                        }
                    }
                }
            }
            Err(e) => self.handle_error(e).await,
        }

        let mut app = self.app.lock().await;
        app.seek_ms.take();
        app.is_fetching_current_playback = false;
    }

    async fn load_playlist_tracks(&mut self, playlist_idk: usize) {
        match self.cloud_music.playlist_tracks(playlist_idk).await {
            Ok(playlist_tracks) => {
                let mut app = self.app.lock().await;

                app.track_table = TrackTable {
                    tracks: playlist_tracks.tracks.clone(),
                    selected_index: 0,
                    context: Some(TrackTableContext::MyPlaylists),
                };
                app.title = String::from("歌曲列表");
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
