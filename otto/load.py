#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from attrdict import AttrDict

from otto.version import OTTO_YML_VERSION

OTTO_DEFAULTS = {
    'jobs': os.cpu_count(),
    'version': OTTO_YML_VERSION,
}

def otto_load(filename='otto.yml'):
    try:
        otto_yml = yaml.safe_load(open(filename))
    except:
        print(f'some error loading the {filename}')
    otto_yml['otto'] = dict(OTTO_DEFAULTS, **otto_yml.get('otto', {}))
    return AttrDict(otto_yml)
