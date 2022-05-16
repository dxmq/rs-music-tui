use crate::model::user::UserProfile;

#[derive(Clone, PartialEq, Debug)]
pub struct LoginInfo {
    pub phone: String,
    pub password: String,
    pub login_state: LoginState,
    pub cancel_login: bool,
    pub is_login_success: bool,
}

impl Default for LoginInfo {
    fn default() -> Self {
        Self {
            phone: "".to_string(),
            password: "".to_string(),
            login_state: Default::default(),
            cancel_login: false,
            is_login_success: false,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResp {
    pub code: usize,
    pub profile: Option<UserProfile>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LoginState {
    NoActive,
    Confirm,
    Cancel,
}

impl Default for LoginState {
    fn default() -> Self {
        Self::NoActive
    }
}

#[derive(Clone, Debug)]
pub struct LoginForm {
    pub phone: String,
    pub password: String,
}
