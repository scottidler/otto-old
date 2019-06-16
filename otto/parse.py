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


def otto_parse(
    args=None, prog=None, desc=None, otto_yml=None, otto_version=None, otto_jobs=None
):
    parser = OttoParser(
        args=args,
        prog=prog,
        desc=desc,
        otto_yml=otto_yml,
        otto_version=otto_version,
        otto_jobs=otto_jobs,
    )
    return parser.parse()


class OttoParser:
    def __init__(
        self,
        args=None,
        prog=None,
        desc=None,
        otto_yml=None,
        otto_version=None,
        otto_jobs=None,
    ):
        self.args = args or sys.argv[1:]
        self.prog = prog or OTTO_PROG
        self.desc = desc or ''
        self.otto_yml = otto_yml or OTTO_YML
        self.otto_version = otto_version or OTTO_VERSION
        self.otto_jobs = otto_jobs or OTTO_JOBS

    def parse(
        self,
        args=None,
        prog=None,
        desc=None,
        otto_yml=None,
        otto_version=None,
        otto_jobs=None,
    ):
        parser = ArgumentParser(
            prog=prog or OTTO_PROG,
            formatter_class=RawDescriptionHelpFormatter,
            add_help=False,
        )
        parser.add_argument(
            '--otto-yml',
            metavar='OTTO-YML',
            default=otto_yml or self.otto_yml,
            help=f'default="%(default)s"; otto yml file to load',
        )
        parser.add_argument(
            '--otto-version',
            metavar='OTTO-VERSION',
            default=otto_version or self.otto_version,
            help=f'default=%(default)s; otto yml version to use',
        )
        parser.add_argument(
            '--otto-jobs',
            metavar='OTTO-JOBS',
            default=otto_jobs or self.otto_jobs,
            help=f'default=%(default)s; number of jobs to use',
        )
        ns, args = parser.parse_known_args(args or self.args)
        cfg, main = otto_load(ns.otto_yml)
        parser = ArgumentParser(
            prog=prog or self.prog, description=desc or self.desc, parents=[parser]
        )
        parser.set_defaults(**ns.__dict__)
        for arg in main.args:
            parser.add_argument(*arg.args, **arg.kwargs)
        ns, rem = parser.parse_known_args(args)
        dbg(ns, rem)
