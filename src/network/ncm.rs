use std::collections::HashMap;
use std::sync::Arc;

use anyhow::private::format_err;
use anyhow::{Error, Result};
use ncmapi::types::{Playlist, UserAccountResp, UserPlaylistResp, UserProfile};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::app::App;
use crate::model::page::Page;
use crate::network::api;

pub type TResult<T> = std::result::Result<T, TError>;
pub type TError = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct CloudMusic {}

impl CloudMusic {
    // pub fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T, Error> {
    //     let result = serde_json::from_str::<T>(input).map_err(|e| {
    //         format_err!(
    //             "convert result failed, reason: {:?}; content: [{:?}]",
    //             e,
    //             input
    //         )
    //     })?;
    //     Ok(result)
    // }

    pub async fn current_user(&self) -> TResult<Option<UserProfile>> {
        let resp = api().user_account().await?;
        let resp = serde_json::from_slice::<UserAccountResp>(resp.data())?;
        Ok(resp.profile)
    }

    pub async fn current_user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
        app: &Arc<Mutex<App>>,
    ) -> TResult<Vec<Playlist>> {
        let mut params = serde_json::Map::new();
        let limit = serde_json::Value::String(limit.into().unwrap_or(50).to_string());
        let offset = serde_json::Value::String(offset.into().unwrap_or(0).to_string());
        params.insert("limit".to_owned(), limit);
        params.insert("offset".to_owned(), offset);

        let app = app.lock().await;
        let params = serde_json::Value::Object(params);
        let resp = api()
            .user_playlist(app.user.as_ref().unwrap().user_id, Some(params))
            .await?;
        let resp = serde_json::from_slice::<UserPlaylistResp>(resp.data())?;
        Ok(resp.playlist)
    }
}
