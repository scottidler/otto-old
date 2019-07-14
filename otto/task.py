#!/usr/bin/env python3
# -*- coding: utf-8 -*-


class OttoParam:
    def __init__(self, name, **kwargs):
        assert isinstance(name, str) and name != ''
        if '-' in name:
            self.args = tuple(name.split('|'))
            self.kind = 'option'
        else:
            self.args = (name,)
            self.kind = 'argument'
            kwargs.pop('help', None)
        self.kwargs = kwargs

    def __repr__(self):
        return f'OttoParam(args={self.args}, kwargs={self.kwargs})'

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
        self.params = params or []
        self.tasks = tasks or []

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
