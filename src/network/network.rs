use std::future::Future;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::app::App;
use crate::event::IoEvent;
use crate::network::api;
use crate::network::ncm::{CloudMusic, TError, TResult};
use anyhow::anyhow;
use ncmapi::types::{Playlist, UserPlaylistResp};

pub struct Network<'a> {
    // 最大搜索限制
    large_search_limit: u32,
    // 最小搜索限制
    small_search_limit: u32,
    pub app: &'a Arc<Mutex<App>>,
    pub ncm: CloudMusic,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Network {
            large_search_limit: 20,
            small_search_limit: 4,
            app,
            ncm: Default::default(),
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetSearchResults(search_term) => {}
            IoEvent::UpdateSearchLimits(large_search_limit, small_search_limit) => {
                self.large_search_limit = large_search_limit;
                self.small_search_limit = small_search_limit;
            }
            IoEvent::GetPlaylists => {
                self.get_current_user_playlists().await;
            }
            IoEvent::GetUser => {
                self.get_user().await;
            }
            _ => {}
        }

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn get_current_user_playlists(&self) {
        let result = self
            .current_user_playlists(self.large_search_limit, None)
            .await;
    }

    pub async fn current_user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
    ) -> TResult<Vec<Playlist>> {
        let mut params = serde_json::Map::new();
        let limit = serde_json::Value::String(limit.into().unwrap_or(50).to_string());
        let offset = serde_json::Value::String(offset.into().unwrap_or(0).to_string());
        params.insert("limit".to_owned(), limit);
        params.insert("offset".to_owned(), offset);

        let app = self.app.lock().await;
        let params = serde_json::Value::Object(params);
        let resp = api()
            .user_playlist(app.user.unwrap().user_id, Some(params))
            .await?;
        let resp = serde_json::from_slice::<UserPlaylistResp>(resp.data())?;
        Ok(resp.playlist)
    }

    async fn get_user(&mut self) {
        match self.ncm.current_user().await {
            Ok(user) => {
                let mut app = self.app.lock().await;
                app.user = user
            }
            Err(e) => self.handle_error(e).await,
        }
    }

    pub async fn handle_error(&mut self, e: TError) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }
}

#[tokio::main]
pub async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
