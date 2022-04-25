use crate::http::crypto::Crypto;
use openssl::hash::{hash, MessageDigest};
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) type Hm = HashMap<String, String>;

#[derive(Serialize, Debug)]
pub struct ApiRequest {
    method: Method,
    url: String,
    data: Option<Value>,
    option: RequestOption,
}

impl ApiRequest {
    #[allow(unused)]
    fn new(method: Method, url: &str) -> Self {
        ApiRequestBuilder::new(method, url).build()
    }

    pub fn pieces(self) -> Pieces {
        (
            self.method,
            self.url,
            self.data,
            self.option.ua,
            self.option.cookies,
            self.option.crypto,
            self.option.api_url,
            self.option.real_ip,
        )
    }

    fn serialize(&self) -> String {
        let a = serde_json::to_string(self).unwrap();
        a
    }

    pub fn id(&self) -> String {
        let digest = hash(MessageDigest::md5(), self.serialize().as_bytes()).unwrap();
        hex::encode(digest)
    }
}

pub struct ApiRequestBuilder {
    config: Config,
}

impl Default for ApiRequestBuilder {
    fn default() -> Self {
        ApiRequestBuilder::new(Method::POST, "")
    }
}

impl ApiRequestBuilder {
    fn new(method: Method, url: &str) -> Self {
        Self {
            config: Config {
                method,
                url: url.to_owned(),
                data: None,
                ua: UA::Chrome,
                cookies: None,
                crypto: Crypto::Weapi,
                api_url: None,
                real_ip: None,
            },
        }
    }

    pub fn build(self) -> ApiRequest {
        let (method, url, data, ua, cookies, crypto, api_url, real_ip) = self.pieces();
        ApiRequest {
            method,
            url,
            data,
            option: RequestOption {
                ua,
                cookies,
                crypto,
                api_url,
                real_ip,
            },
        }
    }

    pub fn pieces(self) -> Pieces {
        let config = self.config;
        (
            config.method,
            config.url,
            config.data,
            config.ua,
            config.cookies,
            config.crypto,
            config.api_url,
            config.real_ip,
        )
    }

    pub fn post(url: &str) -> Self {
        Self::new(Method::POST, url)
    }

    pub fn set_data(mut self, data: Value) -> Self {
        self.config.data = Some(data);
        self
    }

    #[allow(unused)]
    pub fn insert(mut self, key: &str, val: Value) -> Self {
        let mut data = self.config.data.unwrap_or(json!({}));

        data.as_object_mut().unwrap().insert(key.to_owned(), val);
        self.config.data = Some(data);
        self
    }

    pub fn merge(mut self, val: Value) -> Self {
        if !val.is_object() {
            return self;
        }

        let mut data = self.config.data.unwrap_or(json!({}));
        for (k, v) in val.as_object().unwrap() {
            data.as_object_mut()
                .unwrap()
                .insert(k.to_owned(), v.to_owned());
        }
        self.config.data = Some(data);
        self
    }

    #[allow(unused)]
    pub fn set_ua(mut self, ua: UA) -> Self {
        self.config.ua = ua;
        self
    }

    #[allow(unused)]
    pub fn set_cookies(mut self, cookies: Hm) -> Self {
        self.config.cookies = Some(cookies);
        self
    }

    pub fn add_cookie(mut self, name: &str, val: &str) -> Self {
        let mut cookies = self.config.cookies.unwrap_or_default();
        cookies.insert(name.to_owned(), val.to_owned());

        self.config.cookies = Some(cookies);
        self
    }

    pub fn set_crypto(mut self, crypto: Crypto) -> Self {
        self.config.crypto = crypto;
        self
    }

    pub fn set_api_url(mut self, u: &str) -> Self {
        self.config.api_url = Some(String::from(u));
        self
    }

    #[allow(unused)]
    pub fn set_real_ip(mut self, real_ip: &str) -> Self {
        self.config.real_ip = Some(String::from(real_ip));
        self
    }
}

type Pieces = (
    Method,
    String,
    Option<Value>,
    UA,
    Option<Hm>,
    Crypto,
    Option<String>,
    Option<String>,
);

struct Config {
    method: Method,
    url: String,
    data: Option<Value>,
    // options
    ua: UA,
    cookies: Option<Hm>,
    crypto: Crypto,
    api_url: Option<String>,
    real_ip: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
#[allow(unused)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Serialize, Debug)]
pub struct RequestOption {
    ua: UA,
    cookies: Option<Hm>,
    crypto: Crypto,
    api_url: Option<String>,
    real_ip: Option<String>,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[allow(unused)]
pub enum UA {
    Chrome,
    Edge,
    Firefox,
    Safari,
    Android,
    IPhone,
    Linux,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::http::request::{ApiRequestBuilder, Hm, UA};
    use crate::http::route::API_ROUTE;

    type Rb = ApiRequestBuilder;

    #[test]
    fn test_request_builder() {
        let r = Rb::post(API_ROUTE["search"])
            .set_data(json!({
                "name": "alex",
            }))
            .insert("age", json!(19))
            .merge(json!({
                "books": ["book1", "book2"]
            }))
            .set_api_url("/api/url")
            .set_real_ip("real_ip")
            .set_ua(UA::IPhone)
            .set_cookies(Hm::new())
            .add_cookie("sid", "f1h82fg191fh9")
            .build();

        assert_eq!(r.data.unwrap()["age"], 19);
    }

    #[test]
    fn test_serialize() {
        let r = Rb::post(API_ROUTE["search"])
            .set_data(json!({
                "name": "alex",
            }))
            .build();

        let a = r.serialize();
        println!("{}", a)
    }
}
