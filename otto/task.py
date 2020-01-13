#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from leatherman.repr import __repr__

class NotSet():
    def __repr__(self):
        return 'NotSet'

    def __nonzero__(self):
        return False

class OttoParam:
    def __init__(self, uid, value=None, **kwargs):
        assert isinstance(uid, str) and uid != ''
        if '-' in uid:
            self.args = tuple(uid.split('|'))
            self.kind = 'option'
        else:
            self.args = (uid,)
            self.kind = 'argument'
            kwargs.pop('help', None)
        self.name = kwargs.get('name', None)
        if not self.name:
            longest = sorted(self.args, key=len)[-1]
            if longest.startswith('--'):
                self.name = longest[2:]
            elif longest.startswith('-'):
                self.name = longest[1:]
            else:
                self.name = longest
        self.name = self.name.replace('-', '_')
        self.value = value
        self.kwargs = kwargs

    __repr__ = __repr__

    __str__ = __repr__


class OttoTask:
    def __init__(
        self, name, actions, deps=None, uptodate=None, desc=None, params=None, tasks=None
    ):
        assert isinstance(name, str) and name != ''
        assert isinstance(actions, list) and name != []
        self.name = name
        self.actions = actions
        self.deps = deps or []
        self.uptodate = uptodate or []
        self.desc = desc or ''
        self.params = params or {}
        self.tasks = tasks or {}

    __repr__ = __repr__

    __str__ = __repr__
