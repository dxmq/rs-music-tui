use crate::app::{ActiveBlock, App};
use crate::config::UserConfig;
use crate::event;
use crate::event::Key;
use crate::util::SMALL_TERMINAL_HEIGHT;
use anyhow::Result;
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::ExecutableCommand;
use std::io;
use std::io::stdout;
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub(crate) mod draw;
mod help;
pub(crate) mod theme;

pub async fn start_ui(user_config: UserConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    // Terminal initialization
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    if user_config.behavior.set_window_title {
        backend.execute(SetTitle("Netease Cloud music - TUI"))?;
    }

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    loop {
        let mut app = app.lock().await;
        let current_route = app.get_current_route();
        terminal.draw(|mut f| match current_route.active_block {
            // ActiveBlock::HelpMenu => {
            //     draw::draw_help_menu(&mut f, &app);
            // }
            _ => {
                draw::draw_main_layout(&mut f, &app);
            }
        })?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }
            }
            event::Event::Tick => {}
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
    }

    terminal.show_cursor()?;
    close_application()?;
    Ok(())
}

fn close_application() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
