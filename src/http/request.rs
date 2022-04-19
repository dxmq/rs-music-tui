use maybe_async::async_impl;
use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value;

use crate::http::common::{BaseHttpClient, Form, Headers, Query};

#[derive(Debug, Default, Clone)]
pub struct RequestClient {
    pub client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    #[error("response status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

impl RequestClient {
    async fn request<D>(
        &self,
        url: &str,
        method: Method,
        headers: Option<&Headers>,
        params: D,
    ) -> Result<String, RequestError>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut request = self.client.request(method.clone(), url);
        if let Some(headers) = headers {
            request = request.headers(headers.try_into().unwrap());
        }
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
}

#[async_impl]
impl BaseHttpClient for RequestClient {
    type Error = RequestError;

    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        self.request(url, Method::GET, headers, |req| req.query(payload))
            .await
    }

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(url, Method::POST, headers, |req| req.json(payload))
            .await
    }

    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> Result<String, Self::Error> {
        self.request(url, Method::POST, headers, |req| req.form(payload))
            .await
    }

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(url, Method::PUT, headers, |req| req.json(payload))
            .await
    }

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(url, Method::DELETE, headers, |req| req.json(payload))
            .await
    }
}
