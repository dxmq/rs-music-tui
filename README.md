## 一个Rust语言编写的跨平台的终端网易云音乐播放器

### Like this

![](https://s3.bmp.ovh/imgs/2022/07/13/e5c02e7e53ae3647.png)

![](https://s3.bmp.ovh/imgs/2022/07/13/8a4c2d57324a005b.png)

![](https://s3.bmp.ovh/imgs/2022/07/13/44931727507acb47.png)


### Install
> 需要先安装rust运行环境，官网：`https://www.rust-lang.org/zh-CN/`

1. `git clone https://github.com/dxmq/rs-music-tui.git`

2. `cd rs-music-tui`

3. `cargo build --release`，打包构建完后会在`target/release`目录生成可执行文件
> windows环境下`rs-music-tui.exe`，linux 和 macos 下文件为 `rs-music-tui`

4. 运行可执行文件即可


### How to use?
1. 登录
> 使用网易云手机号和密码登录

2. 应用目录
> 在当前系统用户家目录下的`.config/rs-music-tui`目录下
