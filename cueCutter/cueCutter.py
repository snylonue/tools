#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess
from collections import deque
from timetool import *

destr=lambda s,de:s.replace(de,'').strip()

def readCue(content:iter):
	res={'TITLE':deque(),'INDEX':deque(),'FILE':''}
	for x in content:
		if 'TITLE' in x:
			res['TITLE'].append(destr(x,'TITLE'))
		elif 'INDEX 01' in x:
			res['INDEX'].append(destr(x,'INDEX 01'))
		elif 'FILE' in x:
			res['FILE']=destr(x,'FILE').strip(' WAVE')
	try:
		res['TITLE'].popleft()
	except IndexError:
		pass
	return res
