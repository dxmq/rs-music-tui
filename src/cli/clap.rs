use std::io;

use anyhow::{anyhow, Result};
use clap::{App, Arg, Shell};

pub struct ClapApplication {
    pub app: App<'static, 'static>,
}

pub const BANNER: &str =
    "██████╗ ███████╗      ███╗   ███╗██╗   ██╗███████╗██╗ ██████╗      ████████╗██╗   ██╗██╗
██╔══██╗██╔════╝      ████╗ ████║██║   ██║██╔════╝██║██╔════╝      ╚══██╔══╝██║   ██║██║
██████╔╝███████╗█████╗██╔████╔██║██║   ██║███████╗██║██║     █████╗   ██║   ██║   ██║██║
██╔══██╗╚════██║╚════╝██║╚██╔╝██║██║   ██║╚════██║██║██║     ╚════╝   ██║   ██║   ██║██║
██║  ██║███████║      ██║ ╚═╝ ██║╚██████╔╝███████║██║╚██████╗         ██║   ╚██████╔╝██║
╚═╝  ╚═╝╚══════╝      ╚═╝     ╚═╝ ╚═════╝ ╚══════╝╚═╝ ╚═════╝         ╚═╝    ╚═════╝ ╚═╝
                                                                                        ";

impl ClapApplication {
    pub fn new() -> Self {
        ClapApplication {
            app: App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .usage("Press `?` while running the app to see keybindings")
                .before_help(BANNER)
                .after_help("you config path is ...")
                .arg(
                    Arg::with_name("completions")
                        .long("completions")
                        .help("Generates completions for your preferred shell")
                        .takes_value(true)
                        .possible_values(&["bash", "zsh", "fish", "power-shell", "elvish"])
                        .value_name("SHELL"),
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .takes_value(true)
                        .help("netease-cloud-music-tui config path"),
                ),
        }
    }

    pub fn gen_completions(&mut self, completions: &str) -> Result<()> {
        let shell = match completions {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "power-shell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => return Err(anyhow!("no completions available for '{}'", completions)),
        };
        self.app
            .gen_completions_to("cloud-music", shell, &mut io::stdout());
        Ok(())
    }
}
