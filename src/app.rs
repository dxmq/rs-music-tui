use std::sync::mpsc::Sender;

use crate::api::ApiEvent;
use crate::config::UserConfig;

pub struct App {
    pub user_config: UserConfig,
    io_tx: Option<Sender<ApiEvent>>,
}

impl App {
    pub(crate) fn new(io_tx: Sender<ApiEvent>, user_config: UserConfig) -> App {
        App {
            io_tx: Some(io_tx),
            user_config,
        }
    }
}
