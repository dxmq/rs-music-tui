use std::borrow::Cow;

use maybe_async::async_impl;
use reqwest::header::{HeaderMap, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, USER_AGENT};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;
use serde_json::Value;

use crate::http::auth::ClientCookieManager;
use crate::http::common::{BaseHttpClient, Form, Query};

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
        header_map.insert(COOKIE, self.cookie().await.parse().unwrap());

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
}

#[async_impl]
impl BaseHttpClient for RequestClient {
    type Error = RequestError;

    async fn get(&self, url: &str, payload: &Query) -> Result<String, Self::Error> {
        self.request(url, Method::GET, |req| req.query(payload))
            .await
    }

    async fn post(&self, url: &str, payload: &Value) -> Result<String, Self::Error> {
        self.request(url, Method::POST, |req| req.json(payload))
            .await
    }

    async fn post_form<'a>(&self, url: &str, payload: &Form<'a>) -> Result<String, Self::Error> {
        self.request(url, Method::POST, |req| req.form(payload))
            .await
    }

    async fn put(&self, url: &str, payload: &Value) -> Result<String, Self::Error> {
        self.request(url, Method::PUT, |req| req.json(payload))
            .await
    }

    async fn delete(&self, url: &str, payload: &Value) -> Result<String, Self::Error> {
        self.request(url, Method::DELETE, |req| req.json(payload))
            .await
    }
}
