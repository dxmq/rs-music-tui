use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::app::App;
use crate::http::api::CloudMusicApi;
use crate::model::playlist::{Playlist, PlaylistDetail, PlaylistDetailResp, UserPlaylistResp};
use crate::model::table::RecentlyPlayedResp;
use crate::model::track::{Lyric, LyricResp, RecommendedTracksResp, Track, TrackUrl, TrackUrlResp};
use crate::model::user::{LikeTrackIdListResp, UserAccountResp, UserProfile};

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
        // let cache_file_path = app
        //     .user_config
        //     .path_to_config
        //     .as_ref()
        //     .unwrap()
        //     .cache_file_path
        //     .clone();
        // let json_string = std::fs::read_to_string(&cache_file_path);
        // if let Ok(json_string) = json_string {
        //     if let Ok(playlist) = serde_json::from_str::<Vec<Playlist>>(&json_string) {
        //         return Ok(playlist);
        //     }
        // }
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

        // let json_res = serde_json::to_string(&resp.playlist);
        // if let Ok(json) = json_res {
        //     std::fs::write(&cache_file_path, json).unwrap();
        // }
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

    pub async fn recent_song_list(&self, limit: u32) -> Result<Vec<Track>> {
        let resp = self.api.recent_song_list(limit).await.unwrap();
        let resp = serde_json::from_slice::<RecentlyPlayedResp>(resp.data()).unwrap();
        let recently_list = resp.data.list;
        let tracks = recently_list
            .into_iter()
            .map(|item| item.data)
            .collect::<Vec<Track>>();
        Ok(tracks)
    }

    pub async fn recommend_song_list(&self) -> Result<Vec<Track>> {
        let resp = self.api.recommend_song_list().await?;
        let resp = serde_json::from_slice::<RecommendedTracksResp>(resp.data())?;
        if resp.code != 200 {
            return Ok(vec![]);
        }
        match resp.data {
            Some(data) => Ok(data.daily_songs),
            None => Ok(vec![]),
        }
    }

    pub async fn like_track_id_list(&self, user_id: usize) -> Result<HashSet<usize>> {
        let resp = self.api.like_list(user_id).await?;
        let resp = serde_json::from_slice::<LikeTrackIdListResp>(resp.data())?;
        Ok(resp.ids)
    }

    pub async fn lyric(&self, track_id: usize) -> Result<Vec<Lyric>> {
        let resp = self.api.lyric(track_id).await?;
        let resp = serde_json::from_slice::<LyricResp>(resp.data())?;
        if resp.code != 200 {
            return Err(anyhow!("get song lyric failed."));
        }

        let mut lyric: Vec<Lyric> = Vec::new();
        let re = regex::Regex::new(r#"((?:\[\w+:\w+\.\w+\])+)(.*?)$"#).unwrap();
        let re_time = regex::Regex::new(r#"\[(\w+):(\w+)\.(\w+)\]"#).unwrap();
        for s in resp.lrc.lyric.lines() {
            if let Some(cap) = re.captures(&s) {
                let timestamps = cap[1].to_string();
                for t in re_time.captures_iter(&timestamps) {
                    lyric.push(CloudMusic::mk_lyric(cap[2].to_string(), t, 0));
                }
            } else {
                lyric.push(Lyric {
                    lyric: String::new(),
                    timeline: Duration::new(0, 0),
                });
            }
        }
        if !resp.tlyric.lyric.is_empty() {
            for s in resp.tlyric.lyric.lines() {
                if let Some(cap) = re.captures(&s) {
                    let timestamps = cap[1].to_string();
                    for t in re_time.captures_iter(&timestamps) {
                        lyric.push(CloudMusic::mk_lyric(cap[2].to_string(), t, 1));
                    }
                }
            }
        }
        lyric.sort_by(|a, b| a.timeline.cmp(&b.timeline));
        if !lyric.is_empty() {
            return Ok(lyric);
        } else {
            let lyric = vec![Lyric {
                lyric: "no lyric".to_string(),
                timeline: Duration::new(0, 0),
            }];
            Ok(lyric)
        }
    }

    #[allow(unused)]
    fn mk_lyric(value: String, timestamp: regex::Captures, offset: u32) -> Lyric {
        let minute = timestamp[1].parse::<u64>().unwrap_or(0);
        let second = timestamp[2].parse::<u64>().unwrap_or(0);
        let nano = timestamp[3][..1].parse::<u32>().unwrap_or(0) * 10000000;
        let duration_min = minute * 60 + second;
        Lyric {
            lyric: value,
            timeline: Duration::new(duration_min, nano + offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::cloud_music::CloudMusic;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_recommend_song_list() {
        let result = CloudMusic::default().recommend_song_list().await;
        println!("{:#?}", result.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_lyric() {
        let result = CloudMusic::default().lyric(1479526505).await;
        println!("{:#?}", result.unwrap());
    }
}
