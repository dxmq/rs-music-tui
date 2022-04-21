use std::collections::HashSet;
use std::sync::mpsc::Sender;

use crate::config::user_config::UserConfig;
use tui::layout::Rect;

use crate::event::IoEvent;
use crate::model::context::CurrentlyPlaybackContext;
use crate::model::page::Page;
use crate::model::playlist::SimplifiedPlaylist;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Search,
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
    // 播放列表
    pub playlists: Option<Page<SimplifiedPlaylist>>,
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
            if let Err(e) = sender.send(action) {
                self.is_loading = false;
                panic!("Error dispatch event")
            }
        }
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_state(
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
        }
    }
}
