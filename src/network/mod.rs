pub(crate) mod ncm;
pub(crate) mod network;

use crate::config::client_config::ClientConfig;
use ncmapi::NcmApi;
pub use network::start_tokio;

pub fn api() -> NcmApi {
    let config = ClientConfig::default();
    let cookie_path = config.cookie_path.to_str().unwrap();
    NcmApi::new(
        config.cache,
        config.cache_exp,
        config.cache_clean_interval,
        config.preserve_cookies,
        cookie_path,
    )
}
