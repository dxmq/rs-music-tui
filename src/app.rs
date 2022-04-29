use std::collections::HashSet;
use std::ops::Not;
use std::sync::mpsc::Sender;
use std::time::Instant;

use anyhow::Error;
use rand::Rng;
use tui::layout::Rect;
use tui::style::Color;

use crate::config::user_config::UserConfig;
use crate::event::IoEvent;
use crate::model::context::{CurrentlyPlaybackContext, DialogContext};
use crate::model::enums::{PlayingItem, RepeatState, ToggleState};
use crate::model::playlist::Playlist;
use crate::model::table::TrackTable;
use crate::model::track::{Lyric, Track};
use crate::model::user::UserProfile;
use crate::ui::circle::{Circle, CIRCLE, CIRCLE_TICK};

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Library,
    hovered_block: ActiveBlock::Library,
};

pub const LIBRARY_OPTIONS: [&str; 2] = ["最近播放", "每日推荐"];

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
    // 播放列表
    MyPlaylists,
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
    // 当前播放列表索引
    pub selected_playlist_index: Option<usize>,
    pub active_playlist_index: Option<usize>,
    pub help_docs_size: u32,
    pub help_menu_page: u32,
    pub help_menu_max_lines: u32,
    pub help_menu_offset: u32,
    pub home_scroll: u16,
    pub current_playback_context: Option<CurrentlyPlaybackContext>,
    // 歌曲播放进度毫秒
    pub song_progress_ms: u128,
    // 滑动进度毫秒
    pub seek_ms: Option<u128>,
    // 左侧菜单
    pub library: Library,
    // 歌单列表
    pub playlists: Option<Vec<Playlist>>,
    // 歌单偏移量
    pub playlist_offset: u32,
    // 歌单歌曲列表
    pub track_table: TrackTable,
    // 接口错误
    pub api_error: String,
    pub dialog: Option<String>,
    // 对话框选项是否为OK
    pub confirm: bool,
    pub made_for_you_index: usize,
    pub user: Option<UserProfile>,
    pub instant_since_last_current_playback_poll: Instant,
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
    pub playing_circle: Circle,
    pub circle_flag: bool,
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

    // fn poll_current_playback(&mut self) {
    //     // Poll every 5 seconds
    //     let poll_interval_ms = 5_000;
    //
    //     let elapsed = self
    //         .instant_since_last_current_playback_poll
    //         .elapsed()
    //         .as_millis();
    //
    //     if !self.is_fetching_current_playback && elapsed >= poll_interval_ms {
    //         self.is_fetching_current_playback = true;
    //         // Trigger the seek if the user has set a new position
    //         // match self.seek_ms {
    //         //     Some(seek_ms) => self.apply_seek(seek_ms as u32),
    //         //     None => self.dispatch(IoEvent::GetCurrentPlayback),
    //         // }
    //     }
    // }

    pub fn update_on_tick(&mut self) {
        // self.poll_current_playback();
        if let Some(CurrentlyPlaybackContext {
            item: Some(item),
            progress_ms: Some(progress_ms),
            is_playing,
            ..
        }) = &self.current_playback_context
        {
            // Update progress even when the song is not playing,
            // because seeking is possible while paused
            let elapsed = if *is_playing {
                self.instant_since_last_current_playback_poll
                    .elapsed()
                    .as_millis()
            } else {
                0u128
            } + u128::from(*progress_ms);

            let duration_ms = match item {
                PlayingItem::Track(track) => track.duration as u32,
            };

            if elapsed < u128::from(duration_ms) {
                self.song_progress_ms = elapsed;
            } else {
                self.song_progress_ms = duration_ms.into();
            }
            match item {
                PlayingItem::Track(track) => {
                    if track.duration as u128 - self.song_progress_ms < 1000 {
                        let track = track.clone();
                        self.toggle_track(track, ToggleState::Next);
                    }
                }
            }
            if self.get_current_route().active_block == ActiveBlock::Lyric {
                if self.circle_flag {
                    self.playing_circle = Circle {
                        circle: &CIRCLE,
                        color: Color::Reset,
                    }
                } else {
                    self.playing_circle = Circle {
                        circle: &CIRCLE_TICK,
                        color: Color::Cyan,
                    }
                }
                self.circle_flag = !self.circle_flag;
            }
            match &self.lyric {
                Some(lyrics) => {
                    let next_lyric = lyrics.get(self.lyric_index + 1);
                    // check current ms and lyric timeline
                    match next_lyric {
                        Some(next_lyric) => {
                            if self.song_progress_ms as u128 >= next_lyric.timeline.as_millis() {
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
                    if list.tracks.is_empty().not() {
                        let next_index =
                            App::next_index(&list.tracks, Some(list.selected_index), state);
                        let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                        let id = track.id;
                        if next_index != list.tracks.len() {
                            list.selected_index = next_index;
                            self.dispatch(IoEvent::StartPlayback(track));
                        } else if (self.song_progress_ms - track.duration as u128) < 1000 {
                            let mut context = context.clone();
                            context.is_playing = false;
                            self.current_playback_context = Some(context);
                        }
                        self.re_render_lyric(id);
                    }
                }
            }
        }
    }

    pub fn next_or_prev_track(&mut self, state: ToggleState) {
        let mut list = self.my_play_tracks.clone();
        if list.tracks.is_empty().not() {
            let next_index = App::next_index(&list.tracks, Some(list.selected_index), state);
            list.selected_index = next_index;

            let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
            let id = track.id;
            self.dispatch(IoEvent::StartPlayback(track));
            self.re_render_lyric(id);
        }
    }

    fn re_render_lyric(&mut self, track_id: usize) {
        let current_route = self.get_current_route();
        if current_route.id == RouteId::Lyric && current_route.active_block == ActiveBlock::Lyric
            || (current_route.hovered_block == ActiveBlock::Lyric)
        {
            self.dispatch(IoEvent::GetLyric(track_id));
        };
    }

    pub fn shuffle(&mut self) {
        let mut list = self.my_play_tracks.clone();
        let mut rng = rand::thread_rng();
        let next_index = rng.gen_range(0..list.tracks.len());
        list.selected_index = next_index;

        let track = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
        let id = track.id;
        self.dispatch(IoEvent::StartPlayback(track));
        self.re_render_lyric(id);
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
            selected_playlist_index: None,
            active_playlist_index: None,
            playlist_offset: 0,
            help_docs_size: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_menu_offset: 0,
            home_scroll: 0,
            current_playback_context: None,
            song_progress_ms: 0,
            seek_ms: None,
            library: Library { selected_index: 0 },
            playlists: None,
            api_error: String::new(),
            dialog: None,
            confirm: false,
            made_for_you_index: 0,
            user: None,
            track_table: Default::default(),
            instant_since_last_current_playback_poll: Instant::now(),
            is_fetching_current_playback: false,
            large_search_limit: 20,
            volume: 0f32,
            title: String::from("歌曲列表"),
            my_play_tracks: Default::default(),
            liked_track_ids_set: HashSet::new(),
            lyric_index: 0,
            lyric: None,
            playing_circle: Circle::default(),
            circle_flag: true,
        }
    }
}
