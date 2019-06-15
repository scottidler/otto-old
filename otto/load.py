#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError


def get_dict(obj, key, default=None):
    assert isinstance(obj, dict)
    return obj.get(key, default or {})

def get_list(obj, key, default=None):
    assert isinstance(obj, dict)
    return obj.get(key, default or [])

def set_defaults(obj, defaults):
    assert isinstance(obj, dict)
    return dict(defaults, **obj)

def default_args(obj):
    assert isinstance(obj, dict)
    obj['args'] = get_dict(obj, 'args')
    for arg, body in obj['args'].items():
        obj['args'][arg] = set_defaults(body, OPT_DEFAULTS if '-' in arg else POS_DEFAULTS)
    return obj

def default_tasks(obj):
    assert isinstance(obj, dict)
    obj['tasks'] = get_dict(obj, 'tasks')
    for task, body in obj['tasks'].items():
        body = default_args(body)
        obj['tasks'][task] = body
    return obj

def otto_load(otto_yml=OTTO_YML):
    try:
        otto_cfg = yaml.safe_load(open(otto_yml))
    except Exception as ex:
        raise OttoYmlLoadError(otto_yml) from ex
    otto_cfg = default_tasks(otto_cfg)
    return AttrDict(otto_cfg)
