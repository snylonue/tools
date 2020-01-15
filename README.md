# mpv-bilibili
使用mpv播放B站视频  
you-get本身支持调用播放器播放，但不完善

### 安装(使用cargo)
```
git clone https://github.com/snylonue/tools
cd tools/mpv-bilbili
cargo build --release
target/release/b2m.exe -h
```

如果无法编译rust程序，可以使用[Python版(不再维护)](mpv-bilibili/bilibili2mpv.py)

### 用法
```
play bilibili video with mpv

USAGE:
    b2m.exe [FLAGS] <url>

FLAGS:
    -c, --check      check if all dependencies are installed
        --debug      run with stdout from mpv (may not work)
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <url>    video url

```

**依赖**:[you-get](https://github.com/soimort/you-get), [mpv](https://mpv.io)  
**思路**:[Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  

# mpv-unlimit-random
mpv随机播放脚本,使用`y`切换(默认关闭,最后一行可修改快捷键)  
切换文件时打乱播放列表,保持循环播放  
**有几率使播放中的文件位于播放列表末尾导致播放停止** (~~重新切换可恢复,需要修复~~ 已修复)  
**关闭后已经打乱的播放列表无法还原** (~~可能会支持~~)


