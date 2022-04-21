use crate::config::user_config::UserConfig;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct CookieConfig {
    pub cache: bool,
    pub cache_exp: Duration,
    pub cache_clean_interval: Duration,

    pub preserve_cookies: bool,
    pub cookie_path: PathBuf,

    pub log_request: bool,
    pub log_response: bool,
}

impl CookieConfig {
    fn default() -> Self {
        CookieConfig {
            cache: true,
            cache_exp: Duration::from_secs(3 * 60),
            cache_clean_interval: Duration::from_secs(6 * 60),
            preserve_cookies: true,
            cookie_path: UserConfig::cookie_path().unwrap(),
            log_request: false,
            log_response: false,
        }
    }
}
