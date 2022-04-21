use ncmapi::NcmApi;

use crate::config::CookieConfig;

pub struct ApiClient {
    client: NcmApi,
}

impl ApiClient {
    fn new() -> Self {
        let config = CookieConfig::default();
        let path = config.cookie_path.to_str().unwrap();
        Self {
            client: NcmApi::new(
                config.cache,
                config.cache_exp,
                config.cache_clean_interval,
                config.preserve_cookies,
                path,
            ),
        }
    }
}
