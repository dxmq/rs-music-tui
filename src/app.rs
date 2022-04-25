use std::collections::HashSet;
use std::sync::mpsc::Sender;
use std::time::Instant;

use anyhow::Error;
use tui::layout::Rect;

use crate::config::user_config::UserConfig;
use crate::event::IoEvent;
use crate::model::context::{CurrentlyPlaybackContext, DialogContext};
use crate::model::enums::PlayingItem;
use crate::model::playlist::{Playlist, PlaylistDetail};
use crate::model::table::TrackTable;
use crate::model::user::UserProfile;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::MyPlaylists,
    hovered_block: ActiveBlock::Library,
};

pub const LIBRARY_OPTIONS: [&str; 6] = [
    "Made For You",
    "Recently Played",
    "Liked Songs",
    "Albums",
    "Artists",
    "Podcasts",
];

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
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Search,
    MadeForYou,
    Error,
    BasicView,
    Dialog,
    TrackTable,
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
    // 喜欢的歌曲hashset
    pub liked_song_ids_set: HashSet<String>,
    // 歌曲播放进度毫秒
    pub song_progress_ms: u128,
    // 滑动进度毫秒
    pub seek_ms: Option<u128>,
    // 左侧菜单
    pub library: Library,
    // 歌单列表
    // pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub playlists: Option<Vec<Playlist>>,
    pub playlist_offset: u32,
    pub playlist_tracks: Option<PlaylistDetail>,
    // 接口错误
    pub api_error: String,
    pub dialog: Option<String>,
    // 对话框选项是否为OK
    pub confirm: bool,
    pub made_for_you_index: usize,
    pub user: Option<UserProfile>,
    pub track_table: TrackTable,
    pub instant_since_last_current_playback_poll: Instant,
    pub is_fetching_current_playback: bool,
    pub large_search_limit: u32,
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
        }
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
            liked_song_ids_set: HashSet::new(),
            song_progress_ms: 0,
            seek_ms: None,
            library: Library { selected_index: 0 },
            playlists: None,
            playlist_tracks: None,
            api_error: String::new(),
            dialog: None,
            confirm: false,
            made_for_you_index: 0,
            user: None,
            track_table: Default::default(),
            instant_since_last_current_playback_poll: Instant::now(),
            is_fetching_current_playback: false,
            large_search_limit: 20,
        }
    }
}
