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
from otto.exceptions import ParamSpecError
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

    def add_param(self, param):
        return {
            'option': click.Option,
            'argument': click.Argument,
        }[param.kind](param.args, **param.kwargs)

    def add_task(self, task, cfg, add_help_option=True):
        #dbg()

        @click.pass_context
        def callback(ctx, *args, **kwargs):
            dbg()
            cfg['params'] = kwargs

        cmd = click.Group(
            task.name,
            chain=True,
            add_help_option=add_help_option,
            invoke_without_command=True,
            callback=callback,
        )
        #cmd.allow_interspersed_args = True
        cmd.params = [
            self.add_param(param)
            for param in task.params
        ]
        cfg['tasks'] = {
            t.name: dict(params={}, tasks={}, actions={}, deps={})
            for t in task.tasks
        }
        cmd.commands = {
            t.name: self.add_task(t, cfg['tasks'][t.name])
            for t in task.tasks
        }
        return cmd

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
            #dbg()
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
        self.cfg = dict(params={}, tasks={}, actions={}, deps={})
        cmd = self.add_task(otto, self.cfg)
        cmd.params = otto_params + cmd.params
        #dbg(dir=dir(cmd), vars=vars(cmd))
        #dbg(commands=cmd.commands)
        dbg(remainder=self.remainder)
        cmd.main(
            args=self.remainder,
            standalone_mode=False,
            obj=AttrDict(),
        )

        from pprint import pprint
        pprint(dict(cfg=self.cfg))
