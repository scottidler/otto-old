#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError
from otto.task import OttoArg, OttoTask

from leatherman.dbg import dbg

TYPES = {'int': int, 'str': str, 'type': type}


def otto_load(otto_yml=OTTO_YML):
    loader = OttoLoader(otto_yml)
    return loader.load()


class OttoLoader:
    def __init__(self, otto_yml=None):
        self.otto_yml = otto_yml or OTTO_YML

    def load(self, otto_yml=None):
        cfg = OttoLoader.load_cfg(otto_yml or self.otto_yml)
        task = OttoLoader.load_task('otto', cfg)
        return cfg, task

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
    def load_arg(name, cfg):
        if 'choices' in cfg:
            cfg['choies'] = [str(choice) for choice in cfg['choices']]
        if 'type' in cfg:
            cfg['type'] = TYPES[cfg['type']]
        return OttoArg(name, **cfg)

    @staticmethod
    def load_task(name, cfg):
        name = cfg.get('name', name)
        actions = []
        if 'actions' in cfg:
            actions = cfg['actions']
        elif 'action' in cfg:
            actions = [cfg['action']]
        assert isinstance(actions, list)
        args = []
        for arg, body in cfg.get('args', {}).items():
            args += [OttoLoader.load_arg(arg, body)]
        tasks = []
        for task, body in cfg.get('tasks', {}).items():
            tasks += [OttoLoader.load_task(task, body)]
        task = OttoTask(
            name,
            actions,
            deps=OttoLoader.get_val(cfg, 'deps', type=list, default=[]),
            uptodate=OttoLoader.get_val(cfg, 'uptodate', type=list, default=[]),
            desc=OttoLoader.get_val(cfg, 'desc', type=str, default=''),
            args=args,
            tasks=tasks,
        )
        return task
