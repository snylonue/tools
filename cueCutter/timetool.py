#!/usr/bin/env python3
# -*- coding: utf-8 -*-

class TimeLength(object):
	def __init__(self,hour=0,minute=0,second=0):
		self.hour=hour
		self.minute=minute
		self.second=second
		self.simple()
	def __add__(self,other):#add a datetime object is allowed but not recommended
		try:
			new_hour=self.hour+other.hour
			new_minute=self.minute+other.minute
			new_second=self.second+other.second
		except AttributeError:
			return NotImplemented
		return TimeLength(new_hour,new_minute,new_second)
	def __sub__(self,other):
		try:
			new_hour=self.hour-other.hour
			new_minute=self.minute-other.minute
			new_second=self.second-other.second
		except AttributeError:
			return NotImplemented
		return TimeLength(new_hour,new_minute,new_second)
	def __iadd__(self,other):
		try:
			self.hour=other.hour
			self.minute=other.minute
			self.second=other.second
		except ArithmeticError:
			return NotImplemented
	def __isub__(self,other):
		try:
			self.hour-=other.hour
			self.minute-=other.minute
			self.second-=other.second
		except ArithmeticError:
			return NotImplemented
	def __repr__(self):
		return f'{self.hour}:{self.minute}:{self.second}'
	@classmethod
	def fromStr(cls,strtime):
		return cls(*(int(i) for i in strtime.split(':')))
	def simple(self):
		if abs(self.second)>=60:
			self.minute+=self.second//60
			self.second%=60
		if abs(self.minute>=60):
			self.hour+=self.minute//60
			self.minute%=60
		if self.second<0 and self.minute!=0:
			self.second+=60
			self.minute-=1
		if self.minute<0 and self.hour!=0:
			self.minute+=60
			self.hour-=1
	@property
	def seconds(self):
		return self.hour*3600+self.minute*60+self.second
	strftime=__repr__
	strptime=fromStr
