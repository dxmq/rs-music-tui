#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: usize,
    pub nickname: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccountResp {
    pub code: usize,
    pub profile: Option<UserProfile>,
}
