#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError
from otto.task import OttoArg, OttoTask


def otto_load(otto_yml=OTTO_YML):
    loader = OttoLoader(otto_yml)
    return loader.load()


class OttoLoader:
    def __init__(self, otto_yml=None):
        self.otto_yml = otto_yml or OTTO_YML

    def load(self, otto_yml=None):
        cfg = OttoLoader.load_cfg(otto_yml or self.otto_yml)
        args = OttoLoader.load_args(cfg.get('args', {}))
        tasks = OttoLoader.load_tasks(cfg.get('tasks', {}))
        return cfg, args, tasks

    @staticmethod
    def get_val(cfg, key, type=None, default=None):
        val = cfg.get(key, default)
        if type is not None:
            assert isinstance(val, type)
        return val

    @staticmethod
    def load_cfg(otto_yml):
        try:
            cfg = yaml.safe_load(open(otto_yml))
        except Exception as ex:
            raise OttoYmlLoadError(otto_yml) from ex
        return AttrDict(cfg)

    @staticmethod
    def load_args(cfg):
        cfg = AttrDict(cfg)
        args = []
        for name, body in cfg.items():
            arg = OttoArg(name, **body)
            args += [arg]
        return args

    @staticmethod
    def load_tasks(cfg):
        tasks = []
        for name, body in cfg.items():
            actions = body.get('actions', []) or [body.get('action', [])] or []
            assert isinstance(actions, list)
            task = OttoTask(
                name,
                actions,
                deps=OttoLoader.get_val(cfg, 'deps', type=list, default=[]),
                uptodate=OttoLoader.get_val(cfg, 'uptodate', type=list, default=[]),
                desc=OttoLoader.get_val(cfg, 'desc', type=str, default=''),
                args=OttoLoader.load_args(body.get('args', {})),
                tasks=OttoLoader.load_tasks(body.get('tasks', {})),
            )
            tasks += [task]
        return tasks
