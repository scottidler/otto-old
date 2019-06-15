#!/usr/bin/env python3
# -*- coding: utf-8 -*-


class OttoArg:
    def __init__(self, name, **kwargs):
        assert isinstance(name, str) and name != ''
        self.args = name.split('|') if '-' in name else [name]
        self.kwargs = kwargs


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
