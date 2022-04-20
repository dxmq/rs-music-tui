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
