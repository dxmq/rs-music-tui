use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct ApiResponse {
    data: Vec<u8>,
}
impl ApiResponse {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    // pub fn to_string(&self) -> String {
    //     String::from_utf8_lossy(self.data()).to_string()
    // }

    #[allow(unused)]
    pub fn deserialize_to_implict(&self) -> ImplicitResult {
        serde_json::from_slice::<ImplicitResult>(self.data()).unwrap()
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", String::from_utf8_lossy(&*self.data))
    }
}

impl fmt::Debug for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiResponse")
            .field("data", &self.to_string())
            .finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImplicitResult {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub msg: Value,

    #[serde(default)]
    pub message: Value,

    #[serde(default)]
    pub time: usize,

    #[serde(default)]
    pub result: Value,

    #[serde(default)]
    pub data: Value,
}
