use crate::app::App;
use crate::config::UserConfig;
use crate::event;
use anyhow::Result;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, SetTitle};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::sync::{Arc, Mutex};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

async fn start_ui(user_config: UserConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    // Terminal initialization
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    if user_config.behavior.set_window_title {
        backend.execute(SetTitle("spt - Spotify TUI"))?;
    }

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    // play music on, if not send them to the device selection view

    let mut is_first_render = true;

    loop {}

    Ok(())
}
