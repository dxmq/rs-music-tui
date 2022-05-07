use crate::config::keybinds::KeyBindings;

pub fn get_help_docs(key_bindings: &KeyBindings) -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("向下滚动20行"),
            key_bindings.next_page.to_string(),
            String::from("Pagination"),
        ],
        vec![
            String::from("向上滚动20行"),
            key_bindings.previous_page.to_string(),
            String::from("Pagination"),
        ],
        vec![
            String::from("跳转到第一行"),
            key_bindings.jump_to_start.to_string(),
            String::from("Pagination"),
        ],
        vec![
            String::from("跳转到最后一行"),
            key_bindings.jump_to_end.to_string(),
            String::from("Pagination"),
        ],
        vec![
            String::from("Jump to currently playing album"),
            key_bindings.jump_to_album.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("Jump to currently playing artist's album list"),
            key_bindings.jump_to_artist_album.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("跳转到当前播放歌曲行"),
            key_bindings.jump_to_context.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("音量增加10%"),
            key_bindings.increase_volume.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("音量减少10%"),
            key_bindings.decrease_volume.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("下一曲"),
            key_bindings.next_track.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("上一曲"),
            key_bindings.previous_track.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("快进10秒"),
            key_bindings.seek_backwards.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("快退10秒"),
            key_bindings.seek_forwards.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("播放模式切换"),
            key_bindings.repeat.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("移动到左边区块"),
            String::from("h | <Left Arrow Key> | <Ctrl+b>"),
            String::from("General"),
        ],
        vec![
            String::from("移动到下边区块"),
            String::from("j | <Down Arrow Key> | <Ctrl+n>"),
            String::from("General"),
        ],
        vec![
            String::from("移动到上边区块"),
            String::from("k | <Up Arrow Key> | <Ctrl+p>"),
            String::from("General"),
        ],
        vec![
            String::from("移动到右边区块"),
            String::from("l | <Right Arrow Key> | <Ctrl+f>"),
            String::from("General"),
        ],
        vec![
            String::from("移动到列表的最上方"),
            String::from("H"),
            String::from("General"),
        ],
        vec![
            String::from("移动到列表的中间"),
            String::from("M"),
            String::from("General"),
        ],
        vec![
            String::from("移动到列表的最下方"),
            String::from("L"),
            String::from("General"),
        ],
        vec![
            String::from("搜索"),
            key_bindings.search.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("暂停/恢复播放"),
            key_bindings.toggle_playback.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("执行一个操作"),
            String::from("<Enter>"),
            String::from("General"),
        ],
        vec![
            String::from("Go to playbar only screen (basic view)"),
            key_bindings.basic_view.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("返回/直至退出应用"),
            key_bindings.back.to_string(),
            String::from("General"),
        ],
        vec![
            String::from("使用一个区块变为悬浮状态"),
            String::from("<Esc>"),
            String::from("Selected block"),
        ],
        vec![
            String::from("喜欢or不喜欢歌曲/歌曲"),
            String::from("s"),
            String::from("Selected block"),
        ],
        vec![
            String::from("Start playback or enter album/artist/playlist"),
            key_bindings.submit.to_string(),
            String::from("Selected block"),
        ],
        vec![
            String::from("Play all tracks for artist"),
            String::from("e"),
            String::from("Library -> Artists"),
        ],
        vec![
            String::from("发起搜索"),
            String::from("<Enter>"),
            String::from("Search input"),
        ],
        vec![
            String::from("移动光标到左边"),
            String::from("<Left Arrow Key>"),
            String::from("Search input"),
        ],
        vec![
            String::from("移动光标到右边"),
            String::from("<Right Arrow Key>"),
            String::from("Search input"),
        ],
        vec![
            String::from("清除输入"),
            String::from("<Ctrl+l>"),
            String::from("Search input"),
        ],
        vec![
            String::from("删除光标前的搜索内容"),
            String::from("<Ctrl+u>"),
            String::from("Search input"),
        ],
        vec![
            String::from("删除光标后的搜索内容"),
            String::from("<Ctrl+k>"),
            String::from("Search input"),
        ],
        vec![
            String::from("删除之前的单词"),
            String::from("<Ctrl+w>"),
            String::from("Search input"),
        ],
        vec![
            String::from("跳转到搜索内容的开始"),
            String::from("<Ctrl+a>"),
            String::from("Search input"),
        ],
        vec![
            String::from("跳转到搜索内容的末尾"),
            String::from("<Ctrl+e>"),
            String::from("Search input"),
        ],
        vec![
            String::from("搜索框变为悬浮状态"),
            String::from("<Esc>"),
            String::from("Search input"),
        ],
        vec![
            String::from("Delete saved album"),
            String::from("D"),
            String::from("Library -> Albums"),
        ],
        vec![
            String::from("删除一个收藏的歌单"),
            String::from("D"),
            String::from("Playlist"),
        ],
        vec![
            String::from("Follow an artist/playlist"),
            String::from("w"),
            String::from("Search result"),
        ],
        vec![
            String::from("Save (like) album to library"),
            String::from("w"),
            String::from("Search result"),
        ],
        vec![
            String::from("Play random song in playlist"),
            String::from("S"),
            String::from("Selected Playlist"),
        ],
        vec![
            String::from("Add track to queue"),
            key_bindings.add_item_to_queue.to_string(),
            String::from("Hovered over track"),
        ],
    ]
}
