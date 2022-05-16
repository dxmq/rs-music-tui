use std::collections::HashSet;
use std::ops::Not;
use std::panic::PanicInfo;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Error;
use backtrace::Backtrace;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    style::Print,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use tokio::sync::Mutex;
use tokio::try_join;

use crate::app::{ActiveBlock, App, RouteId};
use crate::event::IoEvent;
use crate::handlers::search::{SearchResult, SearchResults, SearchType};
use crate::model::album::{Album, AlbumDetail};
use crate::model::artist::{ArtistBlock, ArtistDetail};
use crate::model::context::{CurrentlyPlaybackContext, TrackTableContext};
use crate::model::enums::{CurrentlyPlayingType, PlayingItem, RepeatState};
use crate::model::login::LoginForm;
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
            IoEvent::GetRecentlyPlayed => {
                self.load_recently_played().await;
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
            IoEvent::GetLyric(track_id, is_active_block) => {
                self.load_track_lyric(track_id, is_active_block).await;
            }
            IoEvent::ToggleLikeTrack(track_id) => {
                self.toggle_like_track(track_id).await;
            }
            IoEvent::GetSearchResults(keyword) => {
                self.load_search_results(&keyword).await;
            }
            IoEvent::ToggleSubscribePlaylist(playlist_id) => {
                self.playlist_subscribe(playlist_id).await;
            }
            IoEvent::SeekForwards => {
                self.seek(true).await;
            }
            IoEvent::SeekBackForwards => {
                self.seek(false).await;
            }
            IoEvent::WebLog(track_id) => {
                self.weblog(track_id).await;
            }
            IoEvent::GetArtistSubList => {
                self.load_artist_sublist().await;
            }
            IoEvent::GetArtistDetail(artist_id, artist_name) => {
                self.load_artist_detail(artist_id, artist_name).await;
            }
            IoEvent::GetAlbumTracks(album) => {
                self.load_album_tracks(album).await;
            }
            IoEvent::ToggleSubscribeArtist(artist_id) => {
                self.toggle_sub_artist(artist_id).await;
            }
            IoEvent::Login(login_form) => {
                self.login_app(login_form).await;
            }
        }

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    pub async fn login_app(&mut self, login_form: LoginForm) {
        // println!("{:?}", login_form);
        match self
            .cloud_music
            .login(login_form.phone.as_str(), login_form.password.as_str())
            .await
        {
            Ok(profile) => {
                let mut app = self.app.lock().await;
                app.login_info.is_login_success = true;
                println!("{:?}", profile);
                app.user = Some(profile);
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn toggle_sub_artist(&mut self, artist_id: usize) {
        match self.cloud_music.artist_sub(artist_id).await {
            Ok(_) => {
                self.load_artist_sublist().await;
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    async fn load_album_tracks(&mut self, album: Box<Album>) {
        match self.cloud_music.album(album.id).await {
            Ok(tracks) => {
                let mut app = self.app.lock().await;
                app.album_detail = Some(AlbumDetail {
                    album: *album,
                    tracks,
                    selected_track_index: 0,
                });
                app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    async fn load_artist_detail(&mut self, artist_id: usize, artist_name: String) {
        let artist_tracks = self.cloud_music.artist_tracks(artist_id);
        let artist_albums = self.cloud_music.artist_albums(artist_id);
        let simi_artists = self.cloud_music.simi_artists(artist_id);

        if let Ok((artist_tracks, artist_albums, simi_artists)) =
            try_join!(artist_tracks, artist_albums, simi_artists)
        {
            let mut app = self.app.lock().await;
            app.artist_detail = Some(ArtistDetail {
                artist_name,
                tracks: artist_tracks,
                albums: artist_albums,
                simi_artists,
                selected_album_index: 0,
                selected_simi_artist_index: 0,
                selected_track_index: 0,
                artist_detail_selected_block: ArtistBlock::Empty,
                artist_detail_hovered_block: ArtistBlock::Tracks,
            });
        }
    }

    async fn load_artist_sublist(&mut self) {
        let mut app = self.app.lock().await;
        match self.cloud_music.artist_sublist().await {
            Ok(artists) => {
                if artists.is_empty().not() {
                    app.artist_sub_ids_set =
                        artists.iter().map(|it| it.id).collect::<HashSet<usize>>();
                }
                app.artists = artists;
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn weblog(&mut self, track_id: usize) {
        self.cloud_music.weblog(track_id).await;
    }

    async fn seek(&mut self, is_forward: bool) {
        let mut app = self.app.lock().await;
        if app.current_playback_context.clone().is_some() {
            let start_time = app.start_time;
            let mut next_duration;
            if is_forward {
                next_duration = start_time.elapsed() + Duration::from_millis(10000);
                if let Some(track_duration) = self.player.get_duration() {
                    if next_duration.as_millis() as u64 > track_duration {
                        next_duration = Duration::from_millis(track_duration)
                    }
                }
                app.start_time = start_time - Duration::from_millis(10000);
            } else if start_time.elapsed() > Duration::from_millis(10000) {
                next_duration = start_time.elapsed() - Duration::from_millis(10000);
                app.start_time = start_time + Duration::from_millis(10000);

                if app.lyric.is_some() {
                    if app.lyric_index <= 5 {
                        app.lyric_index = 0;
                    } else {
                        app.lyric_index -= 5;
                    }
                }
            } else {
                app.lyric_index = 0;
                next_duration = Duration::from_millis(0);
                app.start_time = Instant::now();
            }
            app.song_progress_ms = next_duration.as_millis();
            self.player.seek(next_duration);
        }
    }

    async fn playlist_subscribe(&mut self, playlist_id: usize) {
        let mut app = self.app.lock().await;
        let playlists = app.sub_playlists.clone().unwrap();
        let mut is_subscribe = true;
        for x in playlists {
            if x.id == playlist_id {
                is_subscribe = false;
            }
        }
        let resp = self
            .cloud_music
            .playlist_subscribe(playlist_id, is_subscribe)
            .await;
        match resp {
            Ok(_) => {
                // 重新获取用户歌单
                app.dispatch(IoEvent::GetPlaylists);
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn load_search_results(&mut self, keyword: &str) {
        let search_tracks = self.cloud_music.cloud_search(keyword, SearchType::Track);
        let search_albums = self.cloud_music.cloud_search(keyword, SearchType::Album);
        let search_artists = self.cloud_music.cloud_search(keyword, SearchType::Artist);
        let search_playlists = self.cloud_music.cloud_search(keyword, SearchType::Playlist);

        match try_join!(
            search_tracks,
            search_albums,
            search_artists,
            search_playlists
        ) {
            Ok((
                SearchResult::Tracks(track_results),
                SearchResult::Albums(album_results),
                SearchResult::Artists(artist_results),
                SearchResult::Playlists(playlist_results),
            )) => {
                let mut app = self.app.lock().await;
                app.search_results = SearchResults {
                    tracks: Some(track_results),
                    albums: Some(album_results),
                    artists: Some(artist_results),
                    playlists: Some(playlist_results),
                    ..Default::default()
                }
            }
            Err(e) => {
                self.handle_error(e).await;
            }
            _ => {}
        };
    }

    async fn load_recently_played(&mut self) {
        let mut app = self.app.lock().await;
        let cache_file_path = app.cache_file_path();
        let json_string = std::fs::read_to_string(&cache_file_path);
        let mut table_tracks = vec![];
        if let Ok(json_string) = json_string {
            if !json_string.is_empty() {
                if let Ok(tracks) = serde_json::from_str::<Vec<Track>>(&json_string) {
                    table_tracks = tracks.into_iter().rev().collect();
                }
            }
        }
        let table = TrackTable {
            tracks: table_tracks,
            selected_index: 0,
            context: Some(TrackTableContext::RecentlyPlayed),
        };
        app.track_table = table;
        app.title = "最近播放".to_string();
        app.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable);
    }

    async fn toggle_like_track(&mut self, track_id: usize) {
        let mut app = self.app.lock().await;
        let like_ids_set = app.liked_track_ids_set.clone();
        let like = !like_ids_set.contains(&track_id);
        if let Err(e) = self.cloud_music.toggle_like_track(track_id, like).await {
            app.handle_error(e);
        };
        if like {
            app.liked_track_ids_set.insert(track_id);
        } else {
            app.liked_track_ids_set.remove(&track_id);
        }
    }

    async fn load_track_lyric(&mut self, track_id: usize, is_active_block: bool) {
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
        if is_active_block {
            app.push_navigation_stack(RouteId::Lyric, ActiveBlock::Lyric);
        }
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

    #[allow(unused)]
    async fn load_recently_played_from_cloud_music(&mut self, limit: u32) {
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
                        timestamp: 0,
                        currently_playing_type: CurrentlyPlayingType::Track,
                        repeat_state: RepeatState::Off,
                        item: Some(PlayingItem::Track(track.clone())),
                    };
                    app.current_playback_context = Some(context);
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
                    app.start_time = Instant::now();
                    app.current_playback_context = Some(context);
                    self.player.pause();
                } else {
                    match self.player.get_duration() {
                        Some(_) => {
                            context.is_playing = true;
                            app.start_time = Instant::now();
                            app.current_playback_context = Some(context);
                            self.player.play();
                        }
                        None => {
                            let track = context.item.as_ref().unwrap();
                            let PlayingItem::Track(track) = track;
                            let track_id = track.id;
                            if track_id == 0 {
                                return;
                            }
                            match self.cloud_music.song_url(vec![track.id]).await {
                                Ok(urls) => {
                                    match self.player.play_url(
                                        urls.get(0).cloned().unwrap().url.unwrap().as_str(),
                                    ) {
                                        Ok(()) => {
                                            context.is_playing = true;
                                            app.start_time = Instant::now();
                                            app.current_playback_context = Some(context);

                                            app.dispatch(IoEvent::GetLyric(track_id, false));
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
                    let duration = Duration::from_millis(app.song_progress_ms as u64);
                    app.start_time = Instant::now() - duration;
                }
                app.volume = self.player.get_volume();
            }
            None => {
                self.player.pause();
            }
        }
    }

    async fn start_playback(&mut self, mut track: Track) {
        let mut t = track.clone();
        let track_id = t.id;
        if track_id == 0 {
            return;
        }
        match self.cloud_music.song_url(vec![track_id]).await {
            Ok(urls) => {
                if let Some(track_url) = urls.get(0) {
                    let mut app = self.app.lock().await;
                    if track_url.fee == 1 {
                        if let Some(info) = track_url.free_trial_info.clone() {
                            let duration = (info.end - info.start) * 1000;
                            track.duration = duration;
                            t.duration = duration;
                        }
                    }
                    match self
                        .player
                        .play_url(track_url.url.clone().unwrap().as_str())
                    {
                        Ok(_) => {
                            match app.current_playback_context.clone() {
                                Some(mut context) => {
                                    context.is_playing = true;
                                    context.item = Some(PlayingItem::Track(track));
                                    app.current_playback_context = Some(context);
                                }
                                None => {
                                    let context = CurrentlyPlaybackContext {
                                        is_playing: true,
                                        timestamp: 0,
                                        currently_playing_type: CurrentlyPlayingType::Track,
                                        repeat_state: RepeatState::Off,
                                        item: Some(PlayingItem::Track(track)),
                                    };
                                    app.current_playback_context = Some(context);
                                }
                            }

                            app.start_time = Instant::now();
                            app.volume = self.player.get_volume();
                            self.cache_play_record(t, &mut *app);
                            app.dispatch(IoEvent::GetLyric(track_id, false));
                            app.seek_ms.take();
                            app.is_fetching_current_playback = false;
                        }
                        Err(e) => {
                            app.handle_error(e);
                        }
                    }
                }
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    fn cache_play_record(&mut self, t: Track, app: &mut App) {
        let cache_file_path = app.cache_file_path();
        let json_string = std::fs::read_to_string(&cache_file_path);
        if let Ok(json_string) = json_string {
            if !json_string.is_empty() {
                if let Ok(tracks) = serde_json::from_str::<Vec<Track>>(&json_string) {
                    let mut new_tracks = tracks
                        .into_iter()
                        .filter(|item| item.id != t.id)
                        .collect::<Vec<Track>>();
                    if new_tracks.len() < 500 {
                        new_tracks.push(t);
                    }
                    let new_json = serde_json::to_string(&new_tracks).unwrap_or(String::from(""));
                    if std::fs::write(&cache_file_path, new_json).is_ok() {};
                    return;
                }
            }
        }
        let tracks = vec![t];
        if let Ok(json) = serde_json::to_string(&tracks) {
            if std::fs::write(&cache_file_path, json).is_ok() {}
        }
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
                // 我创建的歌单列表
                let mut my_playlists = vec![];
                // 我收藏的歌单列表
                let mut subscribed_playlists = vec![];
                for (i, play_list) in list.into_iter().enumerate() {
                    if i != 0 {
                        match play_list.subscribed {
                            true => subscribed_playlists.push(play_list),
                            false => my_playlists.push(play_list),
                        }
                    } else {
                        app.my_like_playlist_id = play_list.id;
                    }
                }
                app.playlists = Some(my_playlists);
                app.selected_playlist_index = Some(0);

                app.sub_playlists = Some(subscribed_playlists);
                app.selected_sub_playlist_index = Some(0);
            }
            Err(e) => {
                self.handle_error(e).await;
            }
        }
    }

    async fn load_user(&mut self) {
        let mut app = self.app.lock().await;
        if app.user.is_none() {}
        match self.cloud_music.current_user().await {
            Ok(user) => {
                app.user = user;
            }
            Err(e) => self.handle_error(e).await,
        }
        if app.user.is_some() {
            // 获取最后播放的那条记录
            app.read_current_play_context();
            // 获取喜欢的音乐
            app.dispatch(IoEvent::GetLikeList);
            // 加载歌单列表
            app.dispatch(IoEvent::GetPlaylists);
            // 获取收藏的歌手
            app.dispatch(IoEvent::GetArtistSubList);
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

pub fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        disable_raw_mode().unwrap();
        execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            )),
            DisableMouseCapture
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::model::login::LoginForm;
    use crate::{App, IoEvent, Network, UserConfig};
    use std::sync::{mpsc, Arc};
    use tokio::sync::Mutex;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_login() {
        let (sync_io_tx, _sync_io_rx) = mpsc::channel::<IoEvent>();
        let app: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new(sync_io_tx, UserConfig::new())));
        let mut network = Network::new(&app);
        network
            .login_app(LoginForm {
                phone: "xxx".to_string(),
                password: "xxx".to_string(),
            })
            .await;
    }
}
