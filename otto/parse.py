#!/usr/bin/env python3
# -*- coding: utf-8 -*-
'''
otto
-based cli|task runner
'''

import ast
import sys
import click

from attrdict import AttrDict

from otto.constants import *
from otto.exceptions import ArgSpecError
from otto.load import otto_load

from leatherman.dbg import dbg


def otto_parse(
    args=None, prog=None, desc=None, otto_yml=None, otto_jobs=None, otto_version=None
):
    parser = OttoParser(
        args=args,
        prog=prog,
        desc=desc,
        otto_yml=otto_yml,
        otto_jobs=otto_jobs,
        otto_version=otto_version,
    )
    result = parser.parse()
    return result


class OttoParser:
    def __init__(
        self,
        args=None,
        prog=None,
        desc=None,
        otto_yml=None,
        otto_jobs=None,
        otto_version=None,
    ):
        self.args = args or sys.argv[1:]
        self.prog = prog or OTTO_PROG
        self.desc = desc or OTTO_DESC
        self.otto_yml = otto_yml or OTTO_YML
        self.otto_jobs = otto_jobs or OTTO_JOBS
        self.otto_version = otto_version or OTTO_VERSION

    def cli(self, ctx, *args, **kwargs):
        for key, value in kwargs.items():
            dbg(key, value)
            setattr(self, key, value)

    def resultcallback(self, *args, **kwargs):
        dbg()

    def parse(
        self,
        args=None,
        prog=None,
        desc=None,
        otto_yml=None,
        otto_jobs=None,
        otto_version=None,
    ):
        ns = None

        @click.pass_context
        def otto_callback(ctx, *args, otto_yml=None, otto_jobs=None, otto_version=None, remainder=(), **kwargs):
            dbg()
            self.otto_yml = otto_yml
            self.otto_jobs = otto_jobs
            self.otto_version = otto_version
            self.remainder = remainder
        cli = click.Group(
            prog or self.prog,
            chain=True,
            add_help_option=False,
            invoke_without_command=True,
            callback=otto_callback,
        )
        otto_params = [
            click.Option(
                param_decls=('--otto-yml',),
                metavar='PATH',
                default=otto_yml or self.otto_yml,
                help='otto.yml'),
            click.Option(
                param_decls=('--otto-jobs',),
                metavar='INT',
                default=otto_jobs or self.otto_jobs,
                help='otto num process'),
            click.Option(
                param_decls=('--otto-version',),
                metavar='INT',
                default=otto_version or self.otto_version,
                help='otto version'),
        ]
        cli.params = otto_params + [
            click.Argument(
                param_decls=('remainder',),
                nargs=-1,
                required=False,
            )
        ]
        cli.ignore_unknown_options = True
        cli.main(
            args=args or self.args,
            standalone_mode=False,
            obj=AttrDict(),
        )
        spec, otto = otto_load(otto_yml=self.otto_yml)
        cli.add_help_option = True
        cli.params = otto_params
        for arg, body in spec.otto.args.items():
            if '-' in arg:
                cli.params += [
                    click.Option(
                        param_decls=tuple(arg.split('|')),
                        **body,
                    )
                ]
            else:
                body.pop('help', None)
                cli.params += [
                    click.Argument(
                        param_decls=(arg,),
                        **body,
                    )
                ]
        cli.ignore_unknown_options =False
        cli.allow_interspersed_args=True
        cli.main(
            args=self.remainder,
            standalone_mode=False,
            obj=AttrDict(),
        )
