use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::app::App;
use crate::handlers::search::{
    SearchAlbumResp, SearchArtistResp, SearchPlaylistResp, SearchResult, SearchTrackResp,
    SearchType,
};
use crate::http::api::CloudMusicApi;
use crate::model::album::{Album, AlbumResp};
use crate::model::artist::{
    Artist, ArtistAlbumResp, ArtistSublistResp, ArtistTracksResp, SimiArtistsResp,
};
use crate::model::login::LoginResp;
use crate::model::playlist::{
    Playlist, PlaylistDetail, PlaylistDetailResp, PlaylistTracksResp, UserPlaylistResp,
};
use crate::model::table::RecentlyPlayedResp;
use crate::model::track::{Lyric, LyricResp, RecommendedTracksResp, Track, TrackUrl, TrackUrlResp};
use crate::model::user::{LikeTrackIdListResp, UserAccountResp, UserProfile};

#[derive(Default)]
pub struct CloudMusic {
    api: CloudMusicApi,
}

impl CloudMusic {
    pub async fn current_user(&self) -> Result<Option<UserProfile>> {
        return match self.api.user_account().await {
            Ok(resp) => {
                let resp = serde_json::from_slice::<UserAccountResp>(resp.data())?;
                if resp.code != 200 && resp.profile.is_none() {
                    return Err(anyhow!("网络连接错误"));
                }
                Ok(resp.profile)
            }
            _ => Err(anyhow!("网络连接错误")),
        };
    }

    pub async fn current_user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
        app: &Arc<Mutex<App>>,
    ) -> Result<Vec<Playlist>> {
        let app = app.lock().await;
        let error_msg = "歌单获取失败";
        if app.user.is_none() {
            return Err(anyhow!(error_msg));
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
            .await;
        if let Ok(resp) = resp {
            let resp = serde_json::from_slice::<UserPlaylistResp>(resp.data())?;
            if resp.code == 200 {
                return Ok(resp.playlist);
            }
        }
        Err(anyhow!(error_msg))
    }

    #[allow(unused)]
    pub async fn playlist_tracks(&self, playlist_id: usize) -> Result<PlaylistDetail> {
        let resp = self.api.playlist_detail(playlist_id, None).await;
        if let Ok(resp) = resp {
            let result = serde_json::from_slice::<PlaylistDetailResp>(resp.data())?;
            if result.code == 200 {
                return Ok(result.playlist.unwrap());
            }
        }
        Err(anyhow!("获取歌单歌曲失败"))
    }

    #[allow(unused)]
    pub async fn playlist_page_tracks(
        &self,
        playlist_id: usize,
        offset: usize,
        limit: usize,
    ) -> Result<PlaylistTracksResp> {
        let resp = self.api.playlist_tracks(playlist_id, offset, limit).await;
        if let Ok(resp) = resp {
            let result = serde_json::from_slice::<PlaylistTracksResp>(resp.data())?;
            if result.code == 200 {
                return Ok(result);
            }
        }
        Err(anyhow!("获取歌单歌曲失败"))
    }

    pub async fn song_url(&self, track_id: Vec<usize>) -> Result<Vec<TrackUrl>> {
        if let Ok(resp) = self.api.song_url(&track_id).await {
            let song_url_resp = serde_json::from_slice::<TrackUrlResp>(resp.data())?;
            let track_url = song_url_resp.data.get(0);
            if song_url_resp.code == 200 {
                if let Some(track_url) = track_url {
                    if track_url.url.is_some() {
                        return Ok(song_url_resp.data);
                    }
                }
            }
        }
        return Err(anyhow!("播放失败"));
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
        if let Ok(resp) = self.api.like_list(user_id).await {
            let resp = serde_json::from_slice::<LikeTrackIdListResp>(resp.data())?;
            if resp.code != 200 {
                return Err(anyhow!("网络连接错误"));
            }
            return Ok(resp.ids);
        }
        return Err(anyhow!("网络连接错误"));
    }

    pub async fn lyric(&self, track_id: usize) -> Result<Vec<Lyric>> {
        let resp = self.api.lyric(track_id).await?;
        let resp = serde_json::from_slice::<LyricResp>(resp.data())?;
        if resp.code != 200 {
            return Err(anyhow!("get song lyric failed."));
        }

        let mut lyric: Vec<Lyric> = Vec::new();
        let re = regex::Regex::new(r#"((?:\[\w+:\w+[:\.]\w+\])+)(.*?)$"#).unwrap();
        let re_time = regex::Regex::new(r#"\[(\w+):(\w+)[:\.](\w+)\]"#).unwrap();
        for s in resp.lrc.lyric.lines() {
            if let Some(cap) = re.captures(s) {
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
                if let Some(cap) = re.captures(s) {
                    let timestamps = cap[1].to_string();
                    for t in re_time.captures_iter(&timestamps) {
                        lyric.push(CloudMusic::mk_lyric(cap[2].to_string(), t, 1));
                    }
                }
            }
        }
        lyric.sort_by(|a, b| a.timeline.cmp(&b.timeline));
        if !lyric.is_empty() {
            Ok(lyric)
        } else {
            let lyric = vec![Lyric {
                lyric: "no lyric".to_string(),
                timeline: Duration::new(0, 0),
            }];
            Ok(lyric)
        }
    }

    pub async fn toggle_like_track(&self, track_id: usize, like: bool) -> Result<()> {
        let resp = self.api.like(track_id, like).await;
        match resp {
            Err(e) => {
                return Err(anyhow!(e));
            }
            Ok(resp) => {
                let res = resp.deserialize_to_implict();
                if !res.code == 200 {
                    return Err(anyhow!(if like {
                        "喜欢歌曲失败"
                    } else {
                        "取消喜欢歌曲失败"
                    }));
                }
            }
        }

        Ok(())
    }

    pub async fn cloud_search(
        &self,
        keyword: &str,
        search_type: SearchType,
    ) -> Result<SearchResult> {
        let resp = self.api.cloud_search(keyword, search_type, None).await?;
        match search_type {
            SearchType::Track => {
                let resp = serde_json::from_slice::<SearchTrackResp>(resp.data())?;
                if resp.code == 200 {
                    if let Some(res) = resp.result {
                        return Ok(SearchResult::Tracks(res.songs));
                    }
                }
            }
            SearchType::Album => {
                if let Ok(resp) = serde_json::from_slice::<SearchAlbumResp>(resp.data()) {
                    if resp.code == 200 {
                        if let Some(res) = resp.result {
                            return Ok(SearchResult::Albums(res.albums));
                        }
                    }
                }
            }
            SearchType::Artist => {
                if let Ok(resp) = serde_json::from_slice::<SearchArtistResp>(resp.data()) {
                    if resp.code == 200 {
                        if let Some(res) = resp.result {
                            return Ok(SearchResult::Artists(res.artists));
                        }
                    }
                }
            }
            SearchType::Playlist => {
                if let Ok(resp) = serde_json::from_slice::<SearchPlaylistResp>(resp.data()) {
                    if resp.code == 200 {
                        if let Some(res) = resp.result {
                            return Ok(SearchResult::Playlists(res.playlists));
                        }
                    }
                }
            }
        }
        Ok(SearchResult::new(search_type))
    }

    pub async fn playlist_subscribe(&self, id: usize, is_subscribe: bool) -> Result<()> {
        let resp = self.api.playlist_subscribe(id, is_subscribe).await?;
        let code = resp.deserialize_to_implict().code;
        if code != 200 {
            return Err(anyhow!("收藏歌单失败！"));
        }
        Ok(())
    }

    pub async fn artist_sublist(&self) -> Result<Vec<Artist>> {
        if let Ok(resp) = self.api.artist_sublist().await {
            let res = serde_json::from_slice::<ArtistSublistResp>(resp.data()).unwrap();
            if res.code == 200 {
                return Ok(res.data);
            }
        }
        Err(anyhow!("获取关注歌手失败"))
    }

    #[allow(unused)]
    pub async fn artist_tracks(&self, artist_id: usize) -> Result<Vec<Track>> {
        if let Ok(resp) = self.api.artist_tracks(artist_id).await {
            let resp = serde_json::from_slice::<ArtistTracksResp>(resp.data()).unwrap();
            if resp.code == 200 {
                return Ok(resp.tracks);
            }
        }
        Err(anyhow!("获取歌曲失败"))
    }

    pub async fn artist_albums(&self, artist_id: usize) -> Result<Vec<Album>> {
        if let Ok(resp) = self.api.artist_albums(artist_id).await {
            let resp = serde_json::from_slice::<ArtistAlbumResp>(resp.data()).unwrap();
            if resp.code == 200 {
                return Ok(resp.hot_albums);
            }
        }
        Err(anyhow!("获取相似歌手失败"))
    }

    pub async fn simi_artists(&self, artist_id: usize) -> Result<Vec<Artist>> {
        if let Ok(resp) = self.api.simi_artists(artist_id).await {
            let resp = serde_json::from_slice::<SimiArtistsResp>(resp.data()).unwrap();
            if resp.code == 200 {
                return Ok(resp.artists);
            }
        }
        Err(anyhow!("获取相似歌手失败"))
    }

    pub async fn album(&self, album_id: usize) -> Result<(Vec<Track>, Album)> {
        if let Ok(resp) = self.api.album(album_id).await {
            let resp = serde_json::from_slice::<AlbumResp>(resp.data()).unwrap();
            if resp.code == 200 {
                return Ok((resp.songs, resp.album));
            }
        }
        Err(anyhow!("获取相似歌手失败"))
    }

    pub async fn artist_sub(&self, artist_id: usize) -> Result<()> {
        if let Ok(artist_sublist) = self.artist_sublist().await {
            let mut is_sub: bool = true;
            for x in &artist_sublist {
                if x.id == artist_id {
                    is_sub = false;
                }
            }
            if (self.api.artist_sub(artist_id, is_sub).await).is_ok() {
                return Ok(());
            }
        }
        Err(anyhow!("收藏或取消收藏歌手失败"))
    }

    pub async fn login(&self, phone: &str, password: &str) -> Result<UserProfile> {
        if let Ok(resp) = self.api.login_phone(phone, password).await {
            let resp = serde_json::from_slice::<LoginResp>(resp.data()).unwrap();
            if resp.code == 200 && resp.profile.is_some() {
                return Ok(resp.profile.unwrap());
            }
        }
        return Err(anyhow!("登录失败……"));
    }

    pub async fn weblog(&self, track_id: usize) {
        if (self.api.weblog(track_id).await).is_ok() {};
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
    use pad::{Alignment, PadStr};

    use crate::model::track::Lyric;
    use crate::network::cloud_music::CloudMusic;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_login() {
        let result = CloudMusic::default().login("xxx", "xxx").await;
        println!("{:#?}", result.unwrap());
    }

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

    #[test]
    fn test_match_lyric() {
        let lyric = "[00:00.000] 作词 : Hank\n[00:00.000] 作曲 : DMYoung\n[00:00:00]再来一杯\n[00:08:02] 演唱：Mr.mo/肥皂菌/祈lnory/西瓜Kune/佑可猫/呦猫UNEKO\n[00:13:11] 作曲：DMYoung 作词：Hank\n[00:18:21] 吉他：战场原妖精 混音：刘巍\n[00:23:14]【佑可猫】曾经我觉得我被世间遗忘\n[00:28:23]没有人可以诉说苦闷与悲伤\n[00:34:09]【西瓜Kune】无法去理解人情世故炎凉\n[00:39:18]分不清成长和伪装有什么两样\n[00:45:04]【祈lnory】无处不在的好奇与打量目光\n[00:50:13]【呦猫UNEKO】命令一般的关怀该如何抵抗\n[04:32:08]LaLaLaLa LaLaLa……\n[04:28:23]未来一定就在前方\n[04:23:14]让我们携手再次举杯歌唱\n[01:07:09]…\n[04:18:21]所有情感历经岁月后更闪亮\n[04:13:11]感谢你 给我勇气黑暗中追逐光芒\n[04:10:02]不断鞭策与鼓掌\n[04:07:09] 【合】那些关怀的 那些赞许的\n[04:02:16]感谢你曾经付出陪伴在我身旁\n[03:59:07]并肩闯荡的过往\n[03:56:15] 【佑可猫】那些离去的 那些消失的\n[03:50:13]去燃烧！！\n[03:45:20] 【合】不切实际的梦才值得我们\n[01:47:20]【Inter】\n[02:08:18]【Mr.mo】现在我觉得自己有了方向\n[02:14:03]到处是令人兴奋和惊奇的景象\n[02:19:13] 【西瓜Kune】第一次带着笑容进入梦乡\n[02:24:22]每一天都是那么令人值得期望\n[02:30:08] 【肥皂菌】终于明白成长和伪装不一样\n[02:35:17] 【祈lnory】已不必在意旁人不解的目光\n[02:41:19] 【呦猫UNEKO】不再一个人流浪，有了专属的避风港\n[02:46:12]漫长人生不再漫长\n[02:54:14] 【合】那些温暖的那些热血的\n[02:57:07]那些不自量力的抵抗\n[03:00:00]每一次在柔软后都更令人坚强\n[03:05:09]那些快乐的 那些欢笑的\n[03:08:02]那些无忧无虑时光\n[03:10:18]每一滴泪水在感动后更充满力量\n[03:16:20]所有美好的喜悦勇气和希望\n[03:21:13]已经融入血液，在体内流淌\n[03:26:23]不断温暖我的胸膛\n[03:30:08] 【西瓜Kune】道路越是煎熬 就越坦然面对微笑\n[03:35:17] 【祈lnory】风浪刮得越高，就要越心高气傲\n[03:41:02] 【呦猫UNEKO】世界有太多美好等待寻找\n[05:18:21]【End】\n[00:56.46]【肥皂菌】突破重重的阻挡\n[00:59.04]撕破了所有的伪装\n[01:01.72]终于来到这个地方\n[01:12.78]【合】那些误解的 那些冲动的\n[01:15.26]那些曾经年少轻狂\n[01:18.23]每一次宣泄在悔恨后让人成长\n[01:23.28]那些孤独的 那些迷茫的\n[01:26.08]那些曾经无助彷徨\n[01:28.94]每一次尝试在失败后更充满希望\n[01:34.38]所有的苦痛烦恼忧愁与悲伤\n[01:39.06]随着时间长河静静地流淌\n[01:44.17]不间断地奔向远方\n";
        // let lines = ;
        for s in lyric.lines() {
            // // println!("{}", s);
            let re = regex::Regex::new(r#"((?:\[\w+:\w+[:\.]\w+\])+)(.*?)$"#).unwrap();
            let re_time = regex::Regex::new(r#"\[(\w+):(\w+)[:\.](\w+)\]"#).unwrap();
            // let x: Vec<_> = re.split(s).collect();

            let mut lyric_vec: Vec<Lyric> = Vec::new();
            if let Some(cap) = re.captures(s) {
                let timestamps = cap[1].to_string();
                println!("timestamps--------{}", timestamps);
                let lyric = cap[2].pad_to_width_with_alignment(50, Alignment::Middle);
                println!("lyric--------{}", lyric);
                for t in re_time.captures_iter(&timestamps) {
                    lyric_vec.push(CloudMusic::mk_lyric(cap[2].to_string(), t, 0));
                }
            }
            println!("{:?}", lyric_vec);
        }
    }

    #[test]
    fn padding_char() {
        let s = "I'm over here".pad_to_width_with_alignment(50, Alignment::Middle);
        println!("{}", s);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_playlist_page_tracks() {
        let tracks = CloudMusic::default()
            .playlist_page_tracks(498339500, 0, 10)
            .await;
        println!("{:?}", tracks.unwrap().tracks);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_artist_sublist() {
        let tracks = CloudMusic::default().artist_sublist().await;
        println!("{:?}", tracks.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_artist_tracks() {
        let tracks = CloudMusic::default().artist_tracks(12279635).await;
        println!("{:?}", tracks.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_artist_albums() {
        let albums = CloudMusic::default().artist_albums(12279635).await;
        println!("{:?}", albums.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_simi_artists() {
        let artists = CloudMusic::default().simi_artists(12279635).await;
        println!("{:?}", artists.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_album() {
        let album_tracks = CloudMusic::default().album(32311).await;
        println!("{:?}", album_tracks.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_artist_sub() {
        let res = CloudMusic::default().artist_sub(12279635).await;
        assert!(res.is_ok());
    }
}
