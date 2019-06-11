#!/usr/bin/env python3
# -*- coding: utf-8 -*-

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

    def __repr__(self):
        return f'OttoParam(name={self.name}, value={self.value} args={self.args}, kwargs={self.kwargs})'

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

    def __repr__(self):
        fields = ', '.join(
            [
                f'name={self.name}',
                f'actions={self.actions}',
                f'deps={self.deps}',
                f'uptodate={self.uptodate}',
                f'desc="{self.desc}"',
                f'params={self.params}',
                f'tasks={self.tasks}',
            ]
        )
        return f'OttoTask({fields})'

    __str__ = __repr__
