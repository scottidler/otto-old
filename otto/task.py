#!/usr/bin/env python3
# -*- coding: utf-8 -*-


class OttoArg:
    def __init__(self, name, **kwargs):
        assert isinstance(name, str) and name != ''
        self.args = name.split('|') if '-' in name else [name]
        self.kwargs = kwargs

    def __repr__(self):
        return f'OttoArg(args={self.args}, kwargs={self.kwargs})'

    __str__ = __repr__


class OttoTask:
    def __init__(
        self, name, actions, deps=None, uptodate=None, desc=None, args=None, tasks=None
    ):
        assert isinstance(name, str) and name != ''
        assert isinstance(actions, list) and name != []
        self.name = name
        self.actions = actions
        self.deps = deps or []
        self.uptodate = uptodate or []
        self.desc = desc or ''
        self.args = args or []
        self.tasks = tasks or []

    def __repr__(self):
        fields = ', '.join(
            [
                f'name={self.name}',
                f'actions={self.actions}',
                f'deps={self.deps}',
                f'uptodate={self.uptodate}',
                f'desc="{self.desc}"',
                f'args={self.args}',
                f'tasks={self.tasks}',
            ]
        )
        return f'OttoTask({fields})'

    __str__ = __repr__
