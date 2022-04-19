use maybe_async::maybe_async;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

#[maybe_async]
pub trait BaseHttpClient: Send + Default + Clone + fmt::Debug {
    type Error;

    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Self::Error>;

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error>;

    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> Result<String, Self::Error>;

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error>;

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error>;
}
