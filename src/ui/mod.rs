use std::cmp::{max, min};
use std::io;
use std::io::{stdout, Stdout};
use std::sync::Arc;

use anyhow::Result;
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::ExecutableCommand;
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::app::{ActiveBlock, App, RouteId};
use crate::config::user_config::UserConfig;
use crate::event;
use crate::event::IoEvent;
use crate::event::Key;
use crate::handlers;
use crate::util::SMALL_TERMINAL_HEIGHT;

pub(crate) mod draw;
mod help;

pub async fn start_ui(user_config: UserConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut backend = CrosstermBackend::new(stdout);

    if user_config.behavior.set_window_title {
        backend.execute(SetTitle("Netease Cloud Music - TUI"))?;
    }

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    let mut is_first_render = true;
    loop {
        let mut app = app.lock().await;
        if let Ok(size) = terminal.backend().size() {
            if is_first_render || app.size != size {
                app.help_menu_max_lines = 0;
                app.help_menu_offset = 0;
                app.help_menu_page = 0;

                app.size = size;

                let potential_limit = max((app.size.height as i32) - 13, 0) as u32;
                let max_limit = min(potential_limit, 50);
                let large_search_limit = min((f32::from(size.height) / 1.4) as u32, max_limit);
                let small_search_limit = min((f32::from(size.height) / 2.85) as u32, max_limit / 2);

                app.dispatch(IoEvent::UpdateSearchLimits(
                    large_search_limit,
                    small_search_limit,
                ));

                if app.size.height > 8 {
                    app.help_menu_max_lines = (app.size.height as u32) - 8;
                } else {
                    app.help_menu_max_lines = 0;
                }
            }
        }

        let current_route = app.get_current_route();
        terminal.draw(|f| match current_route.active_block {
            ActiveBlock::HelpMenu => {
                draw::draw_help_menu(f, &app);
            }
            ActiveBlock::Error => {
                draw::draw_error_screen(f, &app);
            }
            ActiveBlock::BasicView => {
                draw::draw_basic_view(f, &app);
            }
            _ => {
                draw::draw_main_layout(f, &app);
            }
        })?;

        if current_route.active_block == ActiveBlock::Input {
            terminal.show_cursor()?;
        } else {
            terminal.hide_cursor()?;
        }
        let cursor_offset = if app.size.height > SMALL_TERMINAL_HEIGHT {
            2
        } else {
            1
        };
        // Put the cursor back inside the input box
        terminal.backend_mut().execute(MoveTo(
            cursor_offset + app.input_cursor_position,
            cursor_offset,
        ))?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }

                let current_active_block = app.get_current_route().active_block;
                if current_active_block == ActiveBlock::Input {
                    handlers::input_handler(key, &mut app);
                } else if key == app.user_config.keys.back {
                    if app.get_current_route().active_block != ActiveBlock::Input {
                        // 不处于搜索输入模式时返回导航堆栈，如果没有更多位置可返回则退出应用程序
                        let pop_result = match app.pop_navigation_stack() {
                            Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
                            Some(x) => Some(x),
                            None => None,
                        };
                        if pop_result.is_none() {
                            // Exit application
                            break;
                        }
                    }
                } else {
                    handlers::handle_app(key, &mut app);
                }
            }
            event::Event::Tick => {
                app.update_on_tick();
            }
        }

        // 如果刚启动（第一次渲染）
        if is_first_render {
            app.dispatch(IoEvent::GetUser);
            // 加载歌单列表
            app.dispatch(IoEvent::GetPlaylists);
            // 获取最后播放的那条记录
            app.read_current_play_context();
            // 获取喜欢的音乐
            app.dispatch(IoEvent::GetLikeList);
            // 获取收藏的歌手
            app.dispatch(IoEvent::GetArtistSubList);
            app.help_docs_size = help::get_help_docs(&app.user_config.keys).len() as u32;
            is_first_render = false;
        }
    }

    close_application(terminal)?;
    Ok(())
}

fn close_application(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
