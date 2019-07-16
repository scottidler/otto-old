#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError
from otto.task import OttoParam, OttoTask

from leatherman.dbg import dbg

TYPES = {'int': int, 'str': str, 'type': type, 'None': None}


def otto_load(otto_yml=OTTO_YML):
    loader = OttoLoader(otto_yml)
    return loader.load()


class OttoLoader:
    def __init__(self, otto_yml=None):
        self.otto_yml = otto_yml or OTTO_YML

    def load(self, otto_yml=None):
        spec = OttoLoader.load_spec(otto_yml or self.otto_yml)
        name, task = OttoLoader.load_task('otto', spec.otto)
        return spec, task

    @staticmethod
    def get_val(spec, key, type=None, default=None):
        val = spec.get(key, default)
        if type is not None:
            assert isinstance(val, type)
        return val

    @staticmethod
    def load_spec(otto_yml):
        try:
            spec = yaml.safe_load(open(otto_yml))
        except Exception as ex:
            raise OttoYmlLoadError(otto_yml) from ex
        return AttrDict(spec)

    @staticmethod
    def load_param(uid, spec):
        if 'choices' in spec:
            spec['choices'] = [str(choice) for choice in spec['choices']]
        if 'type' in spec:
            spec['type'] = TYPES[spec['type']]
        param = OttoParam(uid, **spec)
        return param.name, param

    @staticmethod
    def load_task(uid, spec):
        name = spec.get('name', uid)
        actions = []
        if 'actions' in spec:
            actions = spec['actions']
        elif 'action' in spec:
            actions = [spec['action']]
        assert isinstance(actions, list)
        params = dict([
            OttoLoader.load_param(uid, body)
            for uid, body in spec.get('params', {}).items()
        ])
        tasks = dict([
            OttoLoader.load_task(uid, body)
            for uid, body in spec.get('tasks', {}).items()
        ])
        dbg(tasks)
        task = OttoTask(
            name,
            actions,
            deps=OttoLoader.get_val(spec, 'deps', type=list, default=[]),
            uptodate=OttoLoader.get_val(spec, 'uptodate', type=list, default=[]),
            desc=OttoLoader.get_val(spec, 'desc', type=str, default=''),
            params=params,
            tasks=tasks,
        )
        return task.name, task
