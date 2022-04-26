use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::app::App;
use crate::http::api::CloudMusicApi;
use crate::model::playlist::{Playlist, PlaylistDetail, PlaylistDetailResp, UserPlaylistResp};
use crate::model::table::RecentlyPlayedResp;
use crate::model::track::{Track, TrackUrl, TrackUrlResp};
use crate::model::user::{UserAccountResp, UserProfile};

#[derive(Default)]
pub struct CloudMusic {
    api: CloudMusicApi,
}

impl CloudMusic {
    pub async fn current_user(&self) -> Result<Option<UserProfile>> {
        let resp = self.api.user_account().await?;
        let resp = serde_json::from_slice::<UserAccountResp>(resp.data())?;
        Ok(resp.profile)
    }

    pub async fn current_user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
        app: &Arc<Mutex<App>>,
    ) -> Result<Vec<Playlist>> {
        let app = app.lock().await;
        let cache_file_path = app
            .user_config
            .path_to_config
            .as_ref()
            .unwrap()
            .cache_file_path
            .clone();
        let json_string = std::fs::read_to_string(&cache_file_path);
        if let Ok(json_string) = json_string {
            if let Ok(playlist) = serde_json::from_str::<Vec<Playlist>>(&json_string) {
                return Ok(playlist);
            }
        }

        let mut params = serde_json::Map::new();
        let limit = serde_json::Value::String(limit.into().unwrap_or(50).to_string());
        let offset = serde_json::Value::String(offset.into().unwrap_or(0).to_string());
        params.insert("limit".to_owned(), limit);
        params.insert("offset".to_owned(), offset);
        let params = serde_json::Value::Object(params);

        let resp = self
            .api
            .user_playlist(app.user.as_ref().unwrap().user_id, Some(params))
            .await?;
        let resp = serde_json::from_slice::<UserPlaylistResp>(resp.data())?;

        let json_res = serde_json::to_string(&resp.playlist);
        if let Ok(json) = json_res {
            std::fs::write(&cache_file_path, json).unwrap();
        }
        Ok(resp.playlist)
    }

    pub async fn playlist_tracks(&self, playlist_id: usize) -> Result<PlaylistDetail> {
        let resp = self.api.playlist_detail(playlist_id, None).await?;
        let result = serde_json::from_slice::<PlaylistDetailResp>(resp.data())?;
        Ok(result.playlist.unwrap())
    }

    pub async fn song_url(&self, track_id: Vec<usize>) -> Result<Vec<TrackUrl>> {
        let resp = self.api.song_url(&track_id).await?;
        let song_url_resp = serde_json::from_slice::<TrackUrlResp>(resp.data())?;
        Ok(song_url_resp.data)
    }

    pub async fn recent_song_list(&self) -> Result<Vec<Track>> {
        let api = CloudMusicApi::default();
        let resp = api.recent_song_list().await.unwrap();
        let resp = serde_json::from_slice::<RecentlyPlayedResp>(resp.data()).unwrap();
        let recently_list = resp.data.list;
        let tracks = recently_list
            .into_iter()
            .map(|item| item.data)
            .collect::<Vec<Track>>();
        Ok(tracks)
    }
}
