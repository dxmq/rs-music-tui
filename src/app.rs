use std::collections::HashSet;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Instant;

use anyhow::Error;
use rand::Rng;
use tui::layout::Rect;

use crate::config::user_config::UserConfig;
use crate::event::IoEvent;
use crate::handlers::search::SearchResults;
use crate::model::album::AlbumDetail;
use crate::model::artist::{Artist, ArtistDetail};
use crate::model::context::{CurrentlyPlaybackContext, DialogContext};
use crate::model::dialog::Dialog;
use crate::model::enums::{RepeatState, ToggleState};
use crate::model::login::LoginInfo;
use crate::model::playlist::Playlist;
use crate::model::table::TrackTable;
use crate::model::track::{Lyric, Track};
use crate::model::user::UserProfile;

pub const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Library,
    hovered_block: ActiveBlock::Library,
};

pub const LIBRARY_OPTIONS: [&str; 4] = ["我喜欢", "最近播放", "每日推荐", "关注歌手"];

#[derive(Clone)]
pub struct Library {
    // 当前选中的索引
    pub selected_index: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Home,
    Input,
    Empty,
    Library,
    SearchResultBlock,
    HelpMenu,
    // 播放条
    PlayBar,
    // 我的歌单
    MyPlaylists,
    // 收藏歌单
    SubscribedPlaylists,
    // 错误页
    Error,
    // 基础视图
    BasicView,
    // 对话框
    Dialog(DialogContext),
    MadeForYou,
    // 歌曲表格
    TrackTable,
    // 歌词
    Lyric,
    Artists,
    ArtistDetail,
    AlbumTracks,
    PhoneBlock,
    PasswordBlock,
    LoginButton,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Search,
    #[allow(unused)]
    MadeForYou,
    Error,
    BasicView,
    Dialog,
    TrackTable,
    #[allow(unused)]
    Lyric,
    Artists,
    ArtistDetail,
    AlbumTracks,
    #[allow(unused)]
    PhoneBlock,
    #[allow(unused)]
    PasswordBlock,
    #[allow(unused)]
    LoginButton,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

pub struct App {
    pub user_config: UserConfig,
    io_tx: Option<Sender<IoEvent>>,
    pub(crate) size: Rect,
    pub navigation_stack: Vec<Route>,
    // Inputs:
    // input is the string for input;
    // input_idx is the index of the cursor in terms of character;
    // input_cursor_position is the sum of the width of characters preceding the cursor.
    // Reason for this complication is due to non-ASCII characters, they may
    // take more than 1 bytes to store and more than 1 character width to display.
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    // 是否在加载网络接口数据
    pub is_loading: bool,
    pub help_docs_size: u32,
    pub help_menu_page: u32,
    pub help_menu_max_lines: u32,
    pub help_menu_offset: u32,
    pub home_scroll: u16,
    pub current_playback_context: Option<CurrentlyPlaybackContext>,
    // 播放开始时间
    pub start_time: Instant,
    // 歌曲播放进度毫秒
    pub song_progress_ms: u128,
    // 滑动进度毫秒
    pub seek_ms: Option<u128>,
    // 左侧菜单
    pub library: Library,

    // 创建的歌单列表
    pub playlists: Option<Vec<Playlist>>,
    // 歌单偏移量
    pub playlist_offset: u32,
    // 当前播放列表索引
    pub selected_playlist_index: Option<usize>,
    pub active_playlist_index: Option<usize>,

    pub sub_playlists: Option<Vec<Playlist>>,
    pub sub_playlist_offset: u32,
    pub selected_sub_playlist_index: Option<usize>,
    pub active_sub_playlist_index: Option<usize>,

    // 歌单【我喜欢的音乐】id
    pub my_like_playlist_id: usize,

    // 歌单歌曲列表
    pub track_table: TrackTable,
    // 接口错误
    pub api_error: String,
    pub dialog: Option<Dialog>,
    pub made_for_you_index: usize,
    pub user: Option<UserProfile>,
    pub is_fetching_current_playback: bool,
    pub large_search_limit: u32,
    pub volume: f32,
    pub title: String,
    // 正在播放的歌曲列表
    pub my_play_tracks: TrackTable,
    // 喜欢的歌曲hashset
    pub liked_track_ids_set: HashSet<usize>,
    // 歌词
    pub lyric: Option<Vec<Lyric>>,
    pub lyric_index: usize,
    pub search_results: SearchResults,
    pub artists: Vec<Artist>,
    pub artist_sub_ids_set: HashSet<usize>,
    pub artists_selected_index: usize,
    pub artist_detail: Option<ArtistDetail>,
    pub album_detail: Option<AlbumDetail>,
    pub login_info: LoginInfo,
    // 下一曲播放列表
    pub next_play_tracks: Vec<Track>,
}

impl App {
    pub(crate) fn new(io_tx: Sender<IoEvent>, user_config: UserConfig) -> App {
        App {
            io_tx: Some(io_tx),
            user_config,
            size: Rect::default(),
            ..App::default()
        }
    }

    /// 发送一个网络事件到网络线程
    pub fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Some(sender) = &self.io_tx {
            if sender.send(action).is_err() {
                self.is_loading = false;
                panic!("Error dispatch event");
            }
        }
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let current_route = self.get_current_route_mut();
        if let Some(a) = active_block {
            current_route.active_block = a;
        }
        if let Some(h) = hovered_block {
            current_route.hovered_block = h;
        }
    }

    pub fn push_navigation_stack(
        &mut self,
        next_route_id: RouteId,
        next_active_block: ActiveBlock,
    ) {
        // 防止重复
        if !self
            .navigation_stack
            .last()
            .map(|last_route| last_route.id == next_route_id)
            .unwrap_or(false)
        {
            self.navigation_stack.push(Route {
                id: next_route_id,
                active_block: next_active_block,
                hovered_block: next_active_block,
            })
        }
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() == 1 {
            None
        } else {
            self.navigation_stack.pop()
        }
    }

    pub fn calculate_help_menu_offset(&mut self) {
        let old_offset = self.help_menu_offset;

        if self.help_menu_max_lines < self.help_docs_size {
            self.help_menu_offset = self.help_menu_page * self.help_menu_max_lines;
        }
        if self.help_menu_offset > self.help_docs_size {
            self.help_menu_offset = old_offset;
            self.help_menu_page -= 1;
        }
    }

    pub fn handle_error(&mut self, e: Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.api_error = e.to_string();
    }

    pub fn update_on_tick(&mut self) {
        if let Some(CurrentlyPlaybackContext {
            item: Some(item),
            is_playing,
            ..
        }) = &self.current_playback_context
        {
            if item.id == 0 {
                return;
            }
            let playings = *is_playing;
            let elapsed = if playings {
                self.start_time.elapsed().as_millis()
            } else {
                self.song_progress_ms
            };
            let duration_ms = item.duration as u32;

            if elapsed < u128::from(duration_ms) {
                self.song_progress_ms = elapsed;
            } else {
                self.song_progress_ms = duration_ms.into();
            }
            if item.duration as u128 - self.song_progress_ms < 1000 {
                let track = item.clone();
                // 单曲播放次数+1
                self.dispatch(IoEvent::WebLog(track.id));
                self.toggle_track(track, ToggleState::Next);
            }

            if playings {
                match &self.lyric {
                    Some(lyrics) => {
                        let next_lyric = lyrics.get(self.lyric_index + 1);
                        match next_lyric {
                            Some(next_lyric) => {
                                let timeline = next_lyric.timeline.as_millis();
                                let progress_ms = self.song_progress_ms;
                                if progress_ms as u128 > timeline {
                                    self.lyric_index += 1;
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }

    pub fn toggle_track(&mut self, track: Track, state: ToggleState) {
        if let Some(context) = &self.current_playback_context {
            match context.repeat_state {
                RepeatState::Track => {
                    let id = track.id;
                    self.dispatch(IoEvent::StartPlayback(track));
                    self.re_render_lyric(id);
                }
                RepeatState::Context => {
                    self.next_or_prev_track(state);
                }
                RepeatState::Shuffle => {
                    self.shuffle();
                }
                RepeatState::Off => {
                    let mut list = self.my_play_tracks.clone();
                    if list.tracks.len() > 1 {
                        let mut current_play_track_index = 0;
                        for (i, x) in list.tracks.iter().enumerate() {
                            if x.id == track.id {
                                current_play_track_index = i;
                            }
                        }
                        let next_index =
                            App::next_index(&list.tracks, Some(current_play_track_index), state);

                        let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                        let id = track.id;
                        if next_index != list.tracks.len() {
                            list.selected_index = next_index;
                            self.dispatch(IoEvent::StartPlayback(track));
                            self.re_render_lyric(id);
                        }
                    } else {
                        let mut context = context.clone();
                        self.song_progress_ms = 0;
                        context.is_playing = false;
                        self.current_playback_context = Some(context);
                    }
                }
            }
        }
    }

    pub fn next_or_prev_track(&mut self, state: ToggleState) {
        // let next_tracks = self.next_play_tracks.clone();
        // if !next_tracks.is_empty() {
        //     match state {
        //         ToggleState::Next => {
        //
        //         }
        //         ToggleState::Prev => {
        //
        //         }
        //     }
        // }

        let mut list = self.my_play_tracks.clone();
        if list.tracks.is_empty() {
            return;
        }
        let mut current_play_track_index = 0;
        if let Some(context) = self.current_playback_context.clone() {
            if let Some(item) = context.item {
                for (i, x) in list.tracks.iter().enumerate() {
                    if x.id == item.id {
                        current_play_track_index = i;
                    }
                }
            }
        }
        let next_index = App::next_index(&list.tracks, Some(current_play_track_index), state);
        list.selected_index = next_index;

        let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
        let id = track.id;
        self.dispatch(IoEvent::StartPlayback(track));
        self.re_render_lyric(id);
    }

    #[allow(unused)]
    fn re_render_lyric(&mut self, track_id: usize) {
        let current_route = self.get_current_route();
        if current_route.id == RouteId::Lyric && current_route.active_block == ActiveBlock::Lyric
            || (current_route.hovered_block == ActiveBlock::Lyric)
        {
            self.dispatch(IoEvent::GetLyric(track_id, false));
        };
    }

    pub fn shuffle(&mut self) {
        let mut list = self.my_play_tracks.clone();
        if list.tracks.is_empty().not() {
            let mut rng = rand::thread_rng();
            let next_index = rng.gen_range(0..list.tracks.len());
            list.selected_index = next_index;

            let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
            let id = track.id;
            self.dispatch(IoEvent::StartPlayback(track));
            self.re_render_lyric(id);
        }
    }

    pub fn toggle_play_state(&mut self) {
        let context = self.current_playback_context.clone();
        if let Some(mut context) = context {
            let next_repeat_state = match context.repeat_state {
                RepeatState::Context => RepeatState::Track,
                RepeatState::Track => RepeatState::Shuffle,
                RepeatState::Shuffle => RepeatState::Off,
                RepeatState::Off => RepeatState::Context,
            };
            context.repeat_state = next_repeat_state;
            self.current_playback_context = Some(context);
        }
    }

    pub fn decrease_volume(&mut self) {
        self.dispatch(IoEvent::DecreaseVolume);
    }

    pub fn increase_volume(&mut self) {
        self.dispatch(IoEvent::IncreaseVolume);
    }

    pub fn next_index<T>(
        selection_data: &[T],
        selection_index: Option<usize>,
        state: ToggleState,
    ) -> usize {
        match selection_index {
            Some(selection_index) => {
                if !selection_data.is_empty() {
                    return match state {
                        ToggleState::Next => {
                            let next_index = selection_index + 1;
                            if next_index > selection_data.len() - 1 {
                                0
                            } else {
                                next_index
                            }
                        }
                        ToggleState::Prev => {
                            if selection_index <= 1 {
                                0
                            } else {
                                selection_index - 1
                            }
                        }
                    };
                }
                0
            }
            None => 0,
        }
    }

    pub fn toggle_playback(&mut self) {
        self.dispatch(IoEvent::TogglePlayBack);
    }

    pub fn cache_file_path(&mut self) -> PathBuf {
        let app_dir = self.user_config.get_app_dir();
        let user_id = self.user.clone().unwrap().user_id;
        let cache_file_name = format!("recently_{}.json", user_id);
        app_dir.unwrap().join(cache_file_name)
    }

    pub fn read_current_play_context(&mut self) {
        let cache_file_path = self.cache_file_path();
        let json_string = std::fs::read_to_string(&cache_file_path);
        if let Ok(json_string) = json_string {
            if json_string.is_empty().not() {
                if let Ok(mut tracks) = serde_json::from_str::<Vec<Track>>(&json_string) {
                    let track = tracks.pop();
                    if let Some(track) = track {
                        let context = CurrentlyPlaybackContext::new(Some(track));
                        self.current_playback_context = Some(context);
                        return;
                    }
                }
            }
        }
        self.current_playback_context = Some(CurrentlyPlaybackContext::default());
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            io_tx: None,
            user_config: UserConfig::new(),
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            input: vec![],
            input_idx: 0,
            input_cursor_position: 0,
            is_loading: false,
            playlists: None,
            playlist_offset: 0,
            selected_playlist_index: None,
            active_playlist_index: None,
            sub_playlists: None,
            sub_playlist_offset: 0,
            selected_sub_playlist_index: None,
            active_sub_playlist_index: None,
            my_like_playlist_id: 0,
            help_docs_size: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_menu_offset: 0,
            home_scroll: 0,
            current_playback_context: None,
            song_progress_ms: 0,
            seek_ms: None,
            library: Library { selected_index: 0 },
            api_error: String::new(),
            dialog: None,
            made_for_you_index: 0,
            user: None,
            track_table: Default::default(),
            start_time: Instant::now(),
            is_fetching_current_playback: false,
            large_search_limit: 20,
            volume: 1f32,
            title: String::from("歌曲列表"),
            my_play_tracks: Default::default(),
            liked_track_ids_set: HashSet::new(),
            lyric_index: 0,
            lyric: None,
            search_results: SearchResults::default(),
            artists: vec![],
            artist_sub_ids_set: HashSet::new(),
            artists_selected_index: 0,
            artist_detail: None,
            album_detail: None,
            login_info: Default::default(),
            next_play_tracks: vec![],
        }
    }
}
