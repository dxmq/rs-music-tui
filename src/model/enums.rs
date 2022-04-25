use crate::model::track::Track;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DisallowKey {
    // 中断回播
    InterruptingPlayback,
    // 暂停
    Pausing,
    // 恢复播放
    Resuming,
    // 更改播放进度
    Seeking,
    // 跳过下一个
    SkippingNext,
    // 跳过上一个
    SkippingPrev,
    // 切换重复播放
    TogglingRepeatContext,
    // 切换成随机播放
    TogglingShuffle,
    // 切换重复曲目
    TogglingRepeatTrack,
    // 回放
    TransferringPlayback,
}

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

// 当前播放的类型
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    // 曲目
    Track,
    // 片段
    Episode,
    // 广告
    Advertisement,
    // 未知
    Unknown,
}

// 设备类型
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}

// 播放项
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayingItem {
    // 完整的曲子
    Track(Track),
}

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}
