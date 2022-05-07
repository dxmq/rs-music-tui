use anyhow::Result;
use openssl::hash::{hash, MessageDigest};
use rand::RngCore;
use serde_json::json;
use serde_json::Value;

use crate::handlers::search::SearchType;
use crate::http::client::ApiClient;
use crate::http::crypto::Crypto::Eapi;
use crate::http::request::{ApiRequestBuilder, UA};
use crate::http::response::ApiResponse;
use crate::http::route::API_ROUTE;

#[derive(Default)]
pub struct CloudMusicApi {
    client: ApiClient,
}

impl CloudMusicApi {
    // pub fn new(
    //     enable_cache: bool,
    //     cache_exp: Duration,
    //     cache_clean_interval: Duration,
    //     preserve_cookies: bool,
    //     cookie_path: &str,
    // ) -> Self {
    //     CloudMusicApi {
    //         client: ApiClientBuilder::new(cookie_path)
    //             .cookie_path(cookie_path)
    //             .cache(enable_cache)
    //             .cache_exp(cache_exp)
    //             .cache_clean_interval(cache_clean_interval)
    //             .preserve_cookies(preserve_cookies)
    //             .build()
    //             .unwrap(),
    //     }
    // }

    /// 必选参数 :
    /// phone: 手机号码
    /// password: 密码
    ///
    /// 可选参数 :
    /// countrycode: 国家码，用于国外手机号登录，例如美国传入：1
    /// md5_password: md5加密后的密码,传入后 password 将失效
    pub async fn login_phone(&self, phone: &str, password: &str) -> Result<ApiResponse> {
        let password = md5_hex(password.as_bytes());
        let r = ApiRequestBuilder::post(API_ROUTE["login_cellphone"])
            .add_cookie("os", "pc")
            .set_data(json!({
                "countrycode":   "86",
                "rememberLogin": true,
                "phone": phone,
                "password": password,
            }))
            .build();
        self.client.request(r).await
    }

    /// 说明 : 调用此接口 , 可刷新登录状态
    #[allow(unused)]
    pub async fn login_refresh(&self) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["login_refresh"]).build();

        self.client.request(r).await
    }

    /// 说明 : 调用此接口,可获取登录状态
    #[allow(unused)]
    pub async fn login_status(&self) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["login_status"]).build();

        self.client.request(r).await
    }

    /// 说明 : 调用此接口 , 可退出登录
    #[allow(unused)]
    pub async fn logout(&self) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["logout"]).build();

        self.client.request(r).await
    }

    /// 说明 : 登录后调用此接口 , 传入用户 id, 可以获取用户歌单
    ///
    /// required
    /// 必选参数 : uid : 用户 id
    ///
    /// optional
    /// 可选参数 :
    /// limit : 返回数量 , 默认为 30
    /// offset : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0
    pub async fn user_playlist(&self, uid: usize, opt: Option<Value>) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["user_playlist"])
            .set_data(limit_offset(30, 0))
            .merge(opt.unwrap_or_default())
            .merge(json!({"includeVideo": true, "uid": uid}))
            .build();
        self.client.cache(true).request(r).await
    }

    /// 说明 : 歌单能看到歌单名字, 但看不到具体歌单内容 , 调用此接口 , 传入歌单 id,
    /// 可以获取对应歌单内的所有的音乐(未登录状态只能获取不完整的歌单,登录后是完整的)，
    /// 但是返回的trackIds是完整的，tracks 则是不完整的，
    /// 可拿全部 trackIds 请求一次 song/detail 接口获取所有歌曲的详情
    ///
    /// required
    /// 必选参数 : id : 歌单 id
    ///
    /// optional
    /// 可选参数 : s : 歌单最近的 s 个收藏者,默认为8
    pub async fn playlist_detail(&self, id: usize, opt: Option<Value>) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["playlist_detail"])
            .set_data(json!({"n": 100000, "s": 8, "id": id}))
            .merge(opt.unwrap_or_default())
            .build();

        self.client.request(r).await
    }

    /// 说明 : 使用歌单详情接口后 , 能得到的音乐的 id, 但不能得到的音乐 url, 调用此接口, 传入的音乐 id( 可多个 , 用逗号隔开 ),
    /// 可以获取对应的音乐的 url,未登录状态或者非会员返回试听片段(返回字段包含被截取的正常歌曲的开始时间和结束时间)
    ///
    /// required
    /// 必选参数 : id : 音乐 id
    ///
    /// optional
    /// 可选参数 : br: 码率,默认设置了 999000 即最大码率,如果要 320k 则可设置为 320000,其他类推
    pub async fn song_url(&self, ids: &[usize]) -> Result<ApiResponse> {
        let mut rb = ApiRequestBuilder::post(API_ROUTE["song_url"])
            .set_crypto(Eapi)
            .add_cookie("os", "pc")
            .set_api_url("/api/song/enhance/player/url")
            .set_data(json!({"ids": ids, "br": 999000}));

        if self
            .client
            .cookie("MUSIC_U", self.client.base_url())
            .is_none()
        {
            let mut rng = rand::thread_rng();
            let mut token = [0u8; 16];
            rng.fill_bytes(&mut token);
            rb = rb.add_cookie("_ntes_nuid", &hex::encode(token));
        }

        self.client.cache(false).request(rb.build()).await
    }

    /// 说明 : 登录后调用此接口 ,可获取用户账号信息
    pub async fn user_account(&self) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["user_account"]).build();
        self.client.request(r).await
    }

    // 说明 : 调用此接口 , 可获得最近播放-歌曲
    // 可选参数 : limit : 返回数量 , 默认为 100
    #[allow(unused)]
    pub async fn recent_song_list(&self, limit: u32) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["recent_song_list"])
            .set_data(json!({ "limit": limit }))
            .build();
        self.client.request(r).await
    }

    /// 说明 : 调用此接口 , 可获得每日推荐歌曲 ( 需要登录 )
    pub async fn recommend_song_list(&self) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["recommend_songs"])
            .add_cookie("os", "ios")
            .build();

        self.client.request(r).await
    }

    /// 说明 : 调用此接口 , 传入用户 id, 可获取已喜欢音乐 id 列表(id 数组)
    // 必选参数 : uid: 用户 id
    pub async fn like_list(&self, uid: usize) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["likelist"])
            .set_data(json!({ "uid": uid }))
            .build();

        self.client.request(r).await
    }

    // 说明 : 调用此接口 , 传入音乐 id 可获得对应音乐的歌词
    // 必选参数 : id: 音乐 id
    pub async fn lyric(&self, id: usize) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["lyric"])
            .add_cookie("os", "ios")
            .set_data(json!({ "id": id, "lv": -1, "tv": -1, "kv": -1 }))
            .build();

        self.client.cache(true).request(r).await
    }

    // 记录播放次数
    #[allow(unused)]
    pub async fn weblog(&self, track_id: usize) -> Result<ApiResponse> {
        let data = format!(
            r#"
            [{{
                "action": "play",
                "json": {{
                    "download": 0,
                    "end": "playend",
                    "id": {},
                    "sourceId": "",
                    "time": 240,
                    "type":"song",
                    "wifi": 0
                }}
            }}]
        "#,
            track_id
        );
        let r = ApiRequestBuilder::post(API_ROUTE["weblog"])
            .set_data(json!({"logs": data.to_string()}))
            .set_ua(UA::Android)
            .build();

        self.client.request(r).await
    }

    // 获取用户播放记录
    // 说明 : 登录后调用此接口 , 传入用户 id, 可获取用户播放记录
    //
    // 必选参数 : uid : 用户 id
    //
    // 可选参数 : type : type=1 时只返回 weekData, type=0 时返回 allData
    #[allow(unused)]
    pub async fn play_record(&self, uid: usize, record_type: Option<usize>) -> Result<ApiResponse> {
        let mut r_type = 0;
        if record_type.is_some() {
            r_type = record_type.unwrap();
        }
        let r = ApiRequestBuilder::post(API_ROUTE["user_record"])
            .set_data(json!({ "uid": uid, "type": r_type }))
            .build();

        self.client.request(r).await
    }

    // 必选参数 : id: 歌曲 id
    // 可选参数 : like: 布尔值 , 默认为 true 即喜欢 , 若传 false, 则取消喜欢
    pub async fn like(&self, track_id: usize, like: bool) -> Result<ApiResponse> {
        let r = ApiRequestBuilder::post(API_ROUTE["like"])
            .add_cookie("appver", "2.9.7")
            .add_cookie("os", "pc")
            .set_data(json!({ "trackId": track_id, "like": like , "time": 3, "alg": "itembased"}))
            .build();

        self.client.request(r).await
    }

    /// 说明 : 调用此接口 , 传入搜索关键词可以搜索该音乐 / 专辑 / 歌手 / 歌单 / 用户 , 关键词可以多个 , 以空格隔开 ,
    /// 如 " 周杰伦 搁浅 "( 不需要登录 ), 搜索获取的 mp3url 不能直接用 , 可通过 /song/url 接口传入歌曲 id 获取具体的播放链接
    ///
    /// required
    /// 必选参数 : key: 关键词
    ///
    /// optional
    /// 可选参数 : limit : 返回数量 , 默认为 30 offset : 偏移数量，用于分页 , 如 : 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0
    /// type: 搜索类型；默认为 1 即单曲 , 取值意义 : 1: 单曲, 10: 专辑, 100: 歌手, 1000: 歌单, 1002: 用户, 1004: MV, 1006: 歌词, 1009: 电台, 1014: 视频, 1018:综合
    #[allow(unused)]
    pub async fn cloud_search(
        &self,
        key: &str,
        search_type: SearchType,
        opt: Option<Value>,
    ) -> Result<ApiResponse> {
        let search_type = match search_type {
            SearchType::Track => 1,
            SearchType::Album => 10,
            SearchType::Artist => 100,
            SearchType::Playlist => 1000,
        };
        let r = ApiRequestBuilder::post(API_ROUTE["cloudsearch"])
            .set_data(limit_offset(30, 0))
            .merge(json!({
                "s": key,
                "type": search_type,
            }))
            .merge(opt.unwrap_or_default())
            .build();

        self.client.cache(false).request(r).await
    }

    #[allow(unused)]
    pub async fn playlist_subscribe(&self, id: usize, is_subscribe: bool) -> Result<ApiResponse> {
        let subscribe = if is_subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        };
        let u = replace_all_route_params(API_ROUTE["playlist_subscribe"], subscribe);
        let r = ApiRequestBuilder::post(&u)
            .set_data(json!({ "id": id }))
            .build();
        self.client.request(r).await
    }
}

fn replace_all_route_params(u: &str, rep: &str) -> String {
    let re = regex::Regex::new(r"\$\{.*\}").unwrap();
    re.replace_all(u, rep).to_string()
}

fn md5_hex(pt: &[u8]) -> String {
    hex::encode(hash(MessageDigest::md5(), pt).unwrap())
}

fn limit_offset(limit: usize, offset: usize) -> Value {
    json!({
        "limit": limit,
        "offset": offset
    })
}

#[cfg(test)]
mod tests {
    use crate::handlers::search::{SearchAlbumResp, SearchType};
    use crate::http::api::CloudMusicApi;
    use crate::model::playlist::PlaylistDetailResp;
    use crate::model::table::RecentlyPlayedResp;
    use crate::model::track::LyricResp;
    use crate::model::user::LikeTrackIdListResp;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_login_phone() {
        let api = CloudMusicApi::default();
        let resp = api.login_phone("xxx", "xxxxx").await;
        println!("{:?}", resp);
        assert!(resp.is_ok());

        let res = resp.unwrap().deserialize_to_implict();
        assert_eq!(res.code, 200);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_recent_song() {
        let api = CloudMusicApi::default();
        let resp = api.recent_song_list(10).await.unwrap();
        // println!("{:?}", resp);
        let resp = serde_json::from_slice::<RecentlyPlayedResp>(resp.data()).unwrap();
        let vec = resp.data;
        assert_eq!(vec.list.len(), 10);
        println!("{:?}", vec);

        // let res = resp.unwrap().deserialize_to_implict();
        // assert_eq!(res.code, 200);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_recommend_songs() {
        let api = CloudMusicApi::default();
        let resp = api.recommend_song_list().await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_like_list() {
        let api = CloudMusicApi::default();
        let resp = api.like_list(354192143).await.unwrap();
        let resp = serde_json::from_slice::<LikeTrackIdListResp>(resp.data()).unwrap();

        println!("{:?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_lyric() {
        // 再来一杯 id 527629786 / id 1479526505 犯贱
        let api = CloudMusicApi::default();
        let resp = api.lyric(527629786).await.unwrap();
        let resp = serde_json::from_slice::<LyricResp>(resp.data());
        println!("{:#?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_song_url() {
        let api = CloudMusicApi::default();
        let resp = api.song_url(&[174960]).await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_user_playlists() {
        let api = CloudMusicApi::default();
        let resp = api.user_playlist(354192143, None).await.unwrap();
        // let resp = serde_json::from_slice::<UserPlaylistResp>(resp.data()).unwrap();

        println!("{:#?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_pl() {
        let api = CloudMusicApi::default();
        let resp = api.playlist_detail(498339500, None).await.unwrap();
        let resp = serde_json::from_slice::<PlaylistDetailResp>(resp.data());

        println!("{:#?}", resp.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_weblog() {
        let api = CloudMusicApi::default();
        let resp = api.weblog(527629786).await.unwrap();

        println!("{:#?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_play_record() {
        let api = CloudMusicApi::default();
        let resp = api.play_record(354192143, Some(1)).await.unwrap();
        println!("{:#?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_like() {
        let api = CloudMusicApi::default();
        let resp = api.like(527629786, true).await.unwrap();
        // 我喜欢的歌单id 498339500
        println!("{:#?}", resp);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_cloud_search() {
        let api = CloudMusicApi::default();
        let resp = api
            .cloud_search("不如吃茶去", SearchType::Album, None)
            .await
            .unwrap();
        println!("{}", resp);
        let search_resp = serde_json::from_slice::<SearchAlbumResp>(resp.data()).unwrap();

        println!("{:#?}", search_resp.result.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_playlist_subscribe() {
        let api = CloudMusicApi::default();
        let resp = api.playlist_subscribe(12671414, true).await.unwrap();
        println!("{}", resp);
    }
}
