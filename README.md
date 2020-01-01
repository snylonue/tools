# mpv-bilibili
使用mpv播放B站视频  
### 用法
```
usage: b2m.py [-h] [-d HWDEC] [-o] url

play bilibili video with mpv

positional arguments:
  url                   video url

optional arguments:
  -h, --help            show this help message and exit
  -d HWDEC, --hwdec HWDEC
                        hardware decode opitions,use "mpv --hwdec=help" to get
                        more information
  -o, --output          output mpv information
```
**依赖**:[you-get](https://github.com/soimort/you-get),[mpv](https://mpv.io)  
**思路**:[Linux下用mpv在B站看番（二）：you-get](https://fspark.me/archives/Linux-mpv-bilibili-bangumi-you-get.html)  
**感谢**:[you-get](https://github.com/soimort/you-get),[mpv](https://mpv.io),[@FSpark 's Starry Sky](https://fspark.me/)

# mpv-unlimit-random
mpv随机播放脚本,使用`y`切换(默认关闭,最后一行可修改快捷键)  
切换文件时打乱播放列表,保持循环播放  
**有几率使播放中的文件位于播放列表末尾导致播放停止** (~~重新切换可恢复,需要修复~~ 已修复)  
**关闭后已经打乱的播放列表无法还原** (~~可能会支持~~)


