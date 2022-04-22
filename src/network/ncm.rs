use anyhow::{Error, Result};
use ncmapi::types::{UserAccountResp, UserProfile};

use crate::network::api;

pub type TResult<T> = std::result::Result<T, TError>;
pub type TError = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct CloudMusic {}

impl CloudMusic {
    pub async fn current_user(&self) -> TResult<Option<UserProfile>> {
        let resp = api().user_account().await?;
        let resp = serde_json::from_slice::<UserAccountResp>(resp.data())?;
        Ok(resp.profile)
    }
}
