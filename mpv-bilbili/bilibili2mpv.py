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
	vurls,aurls=[],[]
	for x,v in enumerate(urls):
		if x%2==0:
			vurls.append(v)
		else:
			aurls.append(v)
	#cmd=f'mpv "{''','''.join(vurls)}" --audio-file="{''','''.join(aurls)}" --referrer="https://www.bilibili.com" --no-ytdl --merge-files'
	cmd=f"""mpv "{','.join(vurls)}" --audio-file="{','.join(aurls)}" --referrer="https://www.bilibili.com" --no-ytdl --merge-files \
	--dither=fruit"""
	subprocess.run(cmd,shell=True)

if __name__ == '__main__':
	main(input())