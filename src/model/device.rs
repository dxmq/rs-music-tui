use crate::model::enums::DeviceType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    // 设备id
    pub id: String,
    // 是否激活
    pub is_active: bool,
    // 是否受限制
    pub is_restricted: bool,
    // 设备名
    pub name: String,
    // 设备类型
    #[serde(rename = "type")]
    pub _type: DeviceType,
    // 声音
    pub volume_percent: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}
