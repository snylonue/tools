#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from collections import deque

class Cue(object):
	def __init__(title=None,performed=None,index=None):
		self.__title=title
		self.__performed=performed
		self.__index=index
		self.findTrack=False
	def __bool__(self):
		return not any([i is None for i in [self.__index,self.__performed,self.__title]])
	@property
	def title(self):
		return self.__title
	@property
	def performed(self):
		return self.__performed
	@property
	def index(self):
		return self.__index
	@title.setter
	def title(self,value):
		if self.__title is None:
			raise ValueError('title is already set')
		else:
			self.__title=value
	@performed.setter
	def performed(self,value):
		if self.__performed is None:
			raise ValueError('performed is already set')
		else:
			self.__performed=value
	@index.setter
	def index(self,value):
		if self.__index is None:
			raise ValueError('index is already set')
		else:
			self.__index=value
	@property
	def allNone(self):
		return all([i is None for i in [self.__index,self.__performed,self.__title]])
	
def readcut(l):
	result=deque([])
	temp_cue=Cue()
	for x in l:
		if 'TRACK' in x:
			if temp_cue.allNone:
				temp_cue.findTrack=True
			else:
				raise ValueError
		if 'TITLE' in x:
			if temp_cue.findTrack:
				temp_cue.title=x.strip('TITLE').strip('"')
			else:
				raise ValueError
		if 'PERFORMER' in x:
			if temp_cue.findTrack:
				temp_cue.performed=x.strip('PERFORMER').strip('"')
			else:
				raise ValueError
		if 'INDEX' in x:
			if temp_cue.findTrack:
				temp_cue.index=x.strip('INDEX').strip('"')
			else:
				raise ValueError
		if temp_cue:
			result.append(temp_cue)
			temp_cue=Cue()