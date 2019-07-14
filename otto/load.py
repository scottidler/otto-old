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
        task = OttoLoader.load_task('otto', spec.otto)
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
    def load_param(name, spec):
        if 'choices' in spec:
            spec['choices'] = [str(choice) for choice in spec['choices']]
        if 'type' in spec:
            spec['type'] = TYPES[spec['type']]
        return OttoParam(name, **spec)

    @staticmethod
    def load_task(name, spec):
        name = spec.get('name', name)
        actions = []
        if 'actions' in spec:
            actions = spec['actions']
        elif 'action' in spec:
            actions = [spec['action']]
        assert isinstance(actions, list)
        params = [
            OttoLoader.load_param(param, body)
            for param, body in spec.get('params', {}).items()
        ]
        tasks = [
            OttoLoader.load_task(task, body)
            for task, body in spec.get('tasks', {}).items()
        ]
        task = OttoTask(
            name,
            actions,
            deps=OttoLoader.get_val(spec, 'deps', type=list, default=[]),
            uptodate=OttoLoader.get_val(spec, 'uptodate', type=list, default=[]),
            desc=OttoLoader.get_val(spec, 'desc', type=str, default=''),
            params=params,
            tasks=tasks,
        )
        return task

#    @staticmethod
#    def load_param(spec):
#        args = []
#        for name, body in spec.get('args', []):
#            body['choices'] = [str(choice) for choice in body.get('choices', [])]
#            body['type'] = TYPES[body.get('type', 'None')]
#            args += [OttoParam(
#                OttoParam(name, **body)
#            )]
#
#    @staticmethod
#    def load_tasks(spec):
#        tasks = []
#        for name, body in spec.get('tasks', []):
#            task += [OttoTask(
#                name=body.get('name', name),
#                desc=body.get('desc', None),
#                deps=body.get('deps', []),
#                args=OttoLoader.load_param(body),
#                tasks=OttoLoader.load_tasks(body),
#                actions=body.get('actions', [])
#                uptodate=body.get('uptodate', [])
#            )]
