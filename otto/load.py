#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError

OTTO_DEFAULTS = {
    'jobs': OTTO_JOBS,
    'version': OTTO_VERSION,
}

POS_DEFAULTS = dict(
    action='store',
    nargs=None,
    const=None,
    default=None,
    choices=None,
    metavar=None,
)

OPT_DEFAULTS = dict(POS_DEFAULTS,
    required=False,
    dest=None,
)

def default(defaults, **yml):
    return AttrDict(dict(defaults, **yml))

def get_dict(d, key, default=None):
    return d.get(key, default or {})

def get_list(d, key, default=None):
    return d.get(key, default or [])

def set_defaults(d, defaults):
    return dict(defaults, **d)

def default_otto(d):
    d['otto'] = set_defaults(get_dict(d, 'otto'), OTTO_DEFAULTS)
    return d

def default_args(d):
    d['args'] = get_dict(d, 'args')
    for arg, body in d['args'].items():
        d['args'][arg] = set_defaults(body, OPT_DEFAULTS if '-' in arg else POS_DEFAULTS)
    return d

def default_tasks(d):
    d['tasks'] = get_dict(d, 'tasks')
    for task, body in d['tasks'].items():
        #body = default_opts(body)
        body = default_args(body)
        d['tasks'][task] = body
    return d

def otto_load(filename=OTTO_FILENAME):
    try:
        otto_yml = yaml.safe_load(open(filename))
    except ex:
        raise OttoYmlLoadError(filename) from ex

    otto_yml = default_otto(otto_yml)
    #otto_yml = default_opts(otto_yml)
    otto_yml = default_args(otto_yml)
    otto_yml = default_tasks(otto_yml)
    return AttrDict(otto_yml)
