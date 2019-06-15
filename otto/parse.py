#!/usr/bin/env python3
# -*- coding: utf-8 -*-
'''
otto
yaml-based cli|task runner
'''

import ast
import sys

from argparse import ArgumentParser, RawDescriptionHelpFormatter

from otto.constants import *
from otto.exceptions import ArgSpecError
from otto.load import otto_load

from leatherman.dbg import dbg

TYPES = {'int': int, 'str': str, 'type': type}


def otto_parse(args=None, prog=None, desc=None, otto_yml=None, version=None, jobs=None):
    parser = ArgumentParser(
        prog=prog or OTTO_PROG,
        formatter_class=RawDescriptionHelpFormatter,
        add_help=False,
    )
    parser.add_argument(
        '--otto-yml',
        metavar='OTTO-YML',
        default=otto_yml or OTTO_YML,
        help=f'default="%(default)s"; otto yml file to load',
    )
    parser.add_argument(
        '--otto-version',
        metavar='OTTO-VERSION',
        default=version or OTTO_VERSION,
        help=f'default=%(default)s; otto yml version to use',
    )
    parser.add_argument(
        '--otto-jobs',
        metavar='OTTO-JOBS',
        default=jobs or OTTO_JOBS,
        help=f'default=%(default)s; number of jobs to use',
    )
    ns, args = parser.parse_known_args(args or sys.argv[1:])
    otto_cfg = otto_load(ns.otto_yml)
    parser = ArgumentParser(
        prog=prog or OTTO_PROG, description=desc or __doc__, parents=[parser]
    )
    parser.set_defaults(**ns.__dict__)
    for arg, body in otto_cfg.args.items():
        if 'type' in body:
            body['type'] = TYPES[body['type']]
        if 'choices' in body:
            body['choices'] = [str(choice) for choice in body['choices']]
        parser.add_argument(*divine(arg), **body)
    ns, rem = parser.parse_known_args(args)
    dbg(ns, rem)
