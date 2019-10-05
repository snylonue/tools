#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess
import argparse

def main(url,hwdec,output):
	rescmd=subprocess.run(f'you-get {url} -u',shell=True,stdout=subprocess.PIPE)
	getout=rescmd.stdout.decode().strip().split('\r\n')
	try:
		sp=getout.index('Real URLs:')
	except ValueError:
		raise OSError(f'can not get real url of {url} ')
	else:
		urls=getout[sp+1:]
	assert urls
	if len(urls)==2:
		cmd=f"""mpv{output} "{urls[0]}" --audio-file="{urls[1]}" --referrer="https://www.bilibili.com" --no-ytdl \
--dither=fruit --hwdec={hwdec}"""
	else:
		cmd=f"""mpv{output} {'"'+'" "'.join(urls)+'"'} --referrer="https://www.bilibili.com" --no-ytdl --merge-files \
--dither=fruit --hwdec={hwdec}"""
	assert cmd
	subprocess.run(cmd,shell=True)

if __name__ == '__main__':
	parser=argparse.ArgumentParser(description='play bilibili vedio with mpv')
	parser.add_argument('url',type=str,help='vedio url')
	parser.add_argument('-d','--hwdec',nargs=1,type=str,default=['no'],help='hardware decode opitions,use "mpv --hwdec=help" \
to get more information')
	parser.add_argument('-o','--output',action='store_const',default='.exe',const='',help='output mpv info')
	args=parser.parse_args()
	main(url=args.url,hwdec=args.hwdec[0],args.output)