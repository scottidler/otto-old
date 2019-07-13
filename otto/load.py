#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError
from otto.task import OttoArg, OttoTask

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
    def load_arg(name, spec):
        if 'choices' in spec:
            spec['choices'] = [str(choice) for choice in spec['choices']]
        if 'type' in spec:
            spec['type'] = TYPES[spec['type']]
        return OttoArg(name, **spec)

    @staticmethod
    def load_task(name, spec):
        name = spec.get('name', name)
        actions = []
        if 'actions' in spec:
            actions = spec['actions']
        elif 'action' in spec:
            actions = [spec['action']]
        assert isinstance(actions, list)
        args = []
        for arg, body in spec.get('args', {}).items():
            args += [OttoLoader.load_arg(arg, body)]
        tasks = []
        for task, body in spec.get('tasks', {}).items():
            tasks += [OttoLoader.load_task(task, body)]
        task = OttoTask(
            name,
            actions,
            deps=OttoLoader.get_val(spec, 'deps', type=list, default=[]),
            uptodate=OttoLoader.get_val(spec, 'uptodate', type=list, default=[]),
            desc=OttoLoader.get_val(spec, 'desc', type=str, default=''),
            args=args,
            tasks=tasks,
        )
        return task

#    @staticmethod
#    def load_args(spec):
#        args = []
#        for name, body in spec.get('args', []):
#            body['choices'] = [str(choice) for choice in body.get('choices', [])]
#            body['type'] = TYPES[body.get('type', 'None')]
#            args += [OttoArg(
#                OttoArg(name, **body)
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
#                args=OttoLoader.load_args(body),
#                tasks=OttoLoader.load_tasks(body),
#                actions=body.get('actions', [])
#                uptodate=body.get('uptodate', [])
#            )]
