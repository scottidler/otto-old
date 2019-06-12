#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys

from argparse import ArgumentParser

from otto.constants import *
from otto.exceptions import ArgSpecError
from otto.load import otto_load

from leatherman.dbg import dbg

def make_flags(arg):
    if not arg:
        raise ArgSpecError(arg)
    return arg.split('|')

def divine(arg):
    if not arg:
        raise ArgSpecError(arg)
    if '-' in arg:
        return arg.split('|')
    return [arg]

def otto_parse(args=sys.argv[1:], filename=OTTO_FILENAME):
    parser = ArgumentParser()
    parser.add_argument(
        '--otto-yml',
        metavar='FILENAME',
        default=filename,
        help=f'default="{filename}"; otto yml file to load')
    otto_cfg = otto_load(filename)
    for arg, body in otto_cfg.args.items():
        parser.add_argument(*divine(arg),**body)
    ns, rem = parser.parse_known_args(args)
    dbg(ns)
    dbg(rem)

