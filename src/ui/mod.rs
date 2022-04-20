use crate::app::App;
use crate::config::UserConfig;
use crate::event;
use crate::event::Key;
use anyhow::Result;
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

        terminal.draw(|mut f| {
            draw::draw_main_layout(&mut f, &app);
        })?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }
            }
            event::Event::Tick => {}
        }
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
