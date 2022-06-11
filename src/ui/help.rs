use crate::config::keybinds::KeyBindings;

pub fn get_help_docs(key_bindings: &KeyBindings) -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("返回/直至退出应用"),
            key_bindings.back.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("执行一个操作"),
            String::from("<Enter>"),
            String::from("全局"),
        ],
        vec![
            String::from("暂停/恢复播放"),
            key_bindings.toggle_playback.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("显示歌词"),
            key_bindings.show_lyric.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("搜索"),
            key_bindings.search.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("下一曲"),
            key_bindings.next_track.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("上一曲"),
            key_bindings.previous_track.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("音量增加10%"),
            key_bindings.increase_volume.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("音量减少10%"),
            key_bindings.decrease_volume.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("快进10秒"),
            key_bindings.seek_backwards.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("快退10秒"),
            key_bindings.seek_forwards.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("播放模式切换"),
            key_bindings.repeat.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("基础视图"),
            key_bindings.basic_view.to_string(),
            String::from("全局"),
        ],
        vec![
            String::from("使用一个区块变为悬浮状态"),
            String::from("<Esc>"),
            String::from("区块移动"),
        ],
        vec![
            String::from("移动到左边区块"),
            String::from("h 或 <Left Arrow Key>"),
            String::from("区块移动"),
        ],
        vec![
            String::from("移动到下边区块"),
            String::from("j 或 <Down Arrow Key>"),
            String::from("区块移动"),
        ],
        vec![
            String::from("移动到上边区块"),
            String::from("k 或 <Up Arrow Key>"),
            String::from("区块移动"),
        ],
        vec![
            String::from("移动到右边区块"),
            String::from("l 或 <Right Arrow Key>"),
            String::from("区块移动"),
        ],
        vec![
            String::from("跳转到歌手详情"),
            key_bindings.jump_to_artist_detail.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("跳转到当前歌手专辑"),
            key_bindings.jump_to_artist_album.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("喜欢or不喜欢歌曲/歌手/歌手"),
            String::from("s"),
            String::from("列表操作"),
        ],
        vec![
            String::from("添加到待播放队列"),
            key_bindings.add_item_to_queue.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("取消收藏歌单"),
            String::from("D"),
            String::from("列表操作"),
        ],
        vec![
            String::from("向下滚动20行"),
            key_bindings.next_page.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("向上滚动20行"),
            key_bindings.previous_page.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("跳转到第一行"),
            key_bindings.jump_to_start.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("跳转到最后一行"),
            key_bindings.jump_to_end.to_string(),
            String::from("列表操作"),
        ],
        vec![
            String::from("移动到列表的最上方"),
            String::from("H"),
            String::from("列表操作"),
        ],
        vec![
            String::from("移动到列表的中间"),
            String::from("M"),
            String::from("列表操作"),
        ],
        vec![
            String::from("移动到列表的最下方"),
            String::from("L"),
            String::from("列表操作"),
        ],
        vec![
            String::from("发起搜索"),
            String::from("<Enter>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("移动光标到左边"),
            String::from("<Left Arrow Key>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("移动光标到右边"),
            String::from("<Right Arrow Key>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("清除输入"),
            String::from("<Ctrl+l>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("删除光标前的搜索内容"),
            String::from("<Ctrl+u>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("删除光标后的搜索内容"),
            String::from("<Ctrl+k>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("删除之前的单词"),
            String::from("<Ctrl+w>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("跳转到搜索内容的开始"),
            String::from("<Ctrl+a>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("跳转到搜索内容的末尾"),
            String::from("<Ctrl+e>"),
            String::from("搜索框操作"),
        ],
        vec![
            String::from("搜索框变为悬浮状态"),
            String::from("<Esc>"),
            String::from("搜索框操作"),
        ],
    ]
}
