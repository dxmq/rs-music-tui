use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCookieManager {}

impl ClientCookieManager {
    pub async fn get_cookie(&self) -> String {
        // let access_token = match self.token_info {
        //     Some(ref token_info) => {
        //         if !self.is_token_expired(token_info) {
        //             debug!("token info: {:?}", &token_info);
        //             Some(&token_info.access_token)
        //         } else {
        //             None
        //         }
        //     }
        //     None => None,
        // };
        // match access_token {
        //     Some(access_token) => access_token.to_owned(),
        //     None => match self.request_access_token().await {
        //         Some(new_token_info) => {
        //             debug!("token info: {:?}", &new_token_info);
        //             new_token_info.access_token
        //         }
        //         None => String::new(),
        //     },
        // }
        "aaa".to_string()
    }
}
