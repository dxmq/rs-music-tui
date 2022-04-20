use std::borrow::Cow;
use std::collections::HashMap;

use reqwest::header::{HeaderMap, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, USER_AGENT};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;
use serde_json::Value;

use crate::http::auth::ClientCookieManager;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RequestClient {
    pub prefix: String,
    pub cookie: Option<String>,
    pub client_cookie_manager: Option<ClientCookieManager>,
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    #[error("response status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

impl RequestClient {
    pub fn default() -> RequestClient {
        RequestClient {
            prefix: "https://music.163.com".to_owned(),
            cookie: None,
            client_cookie_manager: None,
        }
    }
    #[allow(unused)]
    async fn request<D>(&self, url: &str, method: Method, params: D) -> Result<String, RequestError>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://music.163.com", &url].concat().into();
        }

        let mut request = CLIENT.request(method.clone(), &url.into_owned());
        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        header_map.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.10586".parse().unwrap());
        header_map.insert(HOST, "music.163.com".parse().unwrap());
        header_map.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
        // header_map.insert(COOKIE, self.cookie().await.parse().unwrap());
        header_map.insert(COOKIE, "".parse().unwrap());

        request = request.headers(header_map);
        request = params(request);
        log::info!("请求内容为：{:?}", request);

        // 请求响应
        let response = request.send().await?;
        if response.status().is_success() {
            response.text().await.map_err(Into::into)
        } else {
            Err(RequestError::StatusCode(response))
        }
    }

    #[allow(unused)]
    async fn cookie(&self) -> String {
        let cookie = match self.cookie {
            Some(ref cookie) => cookie.to_owned(),
            None => match self.client_cookie_manager {
                Some(ref client_cookie_manager) => client_cookie_manager.get_cookie().await,
                None => panic!("client credentials manager is none"),
            },
        };
        cookie
    }

    #[allow(unused)]
    async fn get(
        &self,
        url: &str,
        payload: &HashMap<String, String>,
    ) -> Result<String, RequestError> {
        self.request(url, Method::GET, |req| req.query(payload))
            .await
    }

    #[allow(unused)]
    pub(crate) async fn post(&self, url: &str, payload: &Value) -> Result<String, RequestError> {
        self.request(url, Method::POST, |req| req.json(payload))
            .await
    }

    #[allow(unused)]
    async fn post_form(
        &self,
        url: &str,
        payload: &HashMap<String, String>,
    ) -> Result<String, RequestError> {
        self.request(url, Method::POST, |req| req.form(payload))
            .await
    }

    #[allow(unused)]
    async fn put(&self, url: &str, payload: &Value) -> Result<String, RequestError> {
        self.request(url, Method::PUT, |req| req.json(payload))
            .await
    }

    #[allow(unused)]
    async fn delete(&self, url: &str, payload: &Value) -> Result<String, RequestError> {
        self.request(url, Method::DELETE, |req| req.json(payload))
            .await
    }
}
