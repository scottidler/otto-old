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
from otto.load import otto_load, OttoLoader

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
        self.remainder = []

    def add_param(self, param):
        return {
            'option': click.Option,
            'argument': click.Argument,
        }[param.kind](param.args, **param.kwargs)

    def callback(self, task):
        def callback(*args, **kwargs):
            self.remainder = kwargs.pop('remainder', [])
            for key, value in kwargs.items():
                try:
                    task.params[key].value = value
                except Exception as ex:
                    dbg(key, value, ex)
                    for name, param in task.params.items():
                        dbg(name, param)
        return callback

    def otto_task(self, otto_yml=None, otto_jobs=None, otto_version=None):
        spec = {
            'otto' : {
                'params': {
                    '--otto-yml': {
                        'metavar': 'PATH',
                        'default': otto_yml or self.otto_yml,
                        'help': 'specifiy the yml file to use',
                    },
                    '--otto-jobs': {
                        'metavar': 'INT',
                        'default': otto_jobs or self.otto_jobs,
                        'help': 'specify the number of processes to use',
                    },
                    '--otto-version': {
                        'metavar': 'INT',
                        'default': otto_version or self.otto_version,
                        'help': 'specify the version of yml spec to use',
                    },
                }
            }
        }
        return OttoLoader.load_task('otto', spec['otto'])

    def add_otto_cmd(self, task):
        cmd = click.Group(
            task.name,
            chain=True,
            add_help_option=False,
            invoke_without_command=True,
            callback=self.callback(task),
        )
        cmd.params = [
            self.add_param(param)
            for name, param in task.params.items()
        ]
        return cmd

    def add_cmd(self, task, cmd=None):

        if cmd is None:
            cmd = click.Command(
                task.name,
                callback=self.callback(task),
            )
            cmd.params = []
        cmd.params += [
            self.add_param(param)
            for name, param in task.params.items()
        ]
        for name, subtask in task.tasks.items():
            cmd.add_command(
                self.add_cmd(subtask),
                name=subtask.name,
            )
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
        name, otto = self.otto_task(otto_yml, otto_jobs, otto_version)
        cmd = self.add_otto_cmd(otto)
        cmd.params += [
            click.Argument(
                param_decls=('remainder',),
                nargs=-1,
                required=False,
            )
        ]
        cmd.ignore_unknown_options = True
        cmd.main(
            args=args or self.args,
            standalone_mode=False,
            obj=AttrDict(),
        )
        cmd.add_help_option=True
        cmd.params = cmd.params[:-1]
        spec, otto = otto_load(otto_yml=otto.params['otto_yml'].value)
        cmd = self.add_cmd(otto, cmd=cmd)
        cmd.main(
            args=self.remainder,
            standalone_mode=False,
            obj=AttrDict(),
        )
        dbg(otto)
