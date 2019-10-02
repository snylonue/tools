#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess

def main(url):
	rescmd=subprocess.run(f'you-get {url} -u',shell=True,stdout=subprocess.PIPE)
	getout=rescmd.stdout.decode().strip().split('\r\n')
	for x,v in enumerate(getout):
		if v=='Real URLs:':
			urls=getout[x+1:]
			break
	else:
		raise OSError(f'can not get real url of {url} ')
	assert urls
	if len(urls)==2:
		cmd=f"""mpv "{urls[0]}" --audio-file="{urls[1]}" --referrer="https://www.bilibili.com" --no-ytdl \
--dither=fruit"""
	else:
		cmd=f"""mpv {'"'+'" "'.join(urls)+'"'} --referrer="https://www.bilibili.com" --no-ytdl --merge-files \
--dither=fruit"""
	assert cmd
	subprocess.run(cmd,shell=True)

if __name__ == '__main__':
	main(input())