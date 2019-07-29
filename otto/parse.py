#!/usr/bin/env python3
# -*- coding: utf-8 -*-
'''
otto-based cli|task runner
'''

import ast
import sys
import click

from copy import deepcopy
from addict import Dict
from click import Option, Argument

from otto.constants import *
from otto.exceptions import ParamSpecError
from otto.load import otto_load

from leatherman.dbg import dbg


def otto_parse(
    args=None,
    prog=None,
    desc=None,
    otto_yml=None,
    otto_jobs=None,
    otto_version=None
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
        self.cmds = []

    def callback(self, task):
        def callback(*args, **kwargs):
            self.remainder = kwargs.pop('remainder', [])
            if 'action' in task or 'actions' in task:
                self.cmds += [task.name]
            for key, value in kwargs.items():
                try:
                    task.params[key].value = value
                except Exception as ex:
                    dbg(ex)
        return callback

    def otto_spec(self, otto_yml=None, otto_jobs=None, otto_version=None):
        return Dict({
            'otto' : {
                'params': {
                    '--otto-yml': {
                        'metavar': 'PATH',
                        'default': otto_yml or self.otto_yml,
                        'show_default': True,
                        'help': 'specifiy the yml file to use',
                    },
                    '--otto-jobs': {
                        'metavar': 'INT',
                        'default': otto_jobs or self.otto_jobs,
                        'show_default': True,
                        'help': 'specify the number of processes to use',
                    },
                    '--otto-version': {
                        'metavar': 'INT',
                        'default': otto_version or self.otto_version,
                        'show_default': True,
                        'help': 'specify the version of yml spec to use',
                    },
                }
            }
        })

    def add_params(self, params):
        click_params = []
        def otto_first(x):
            return 0 if 'otto' in x else 1
        for uid in sorted(params.keys(), key=otto_first):
            param = params.pop(uid)
            param.name = param.get('name', uid)
            if '-' in uid:
                decls = tuple(uid.split('|'))
                ctor = Option
            else:
                decls = (uid,)
                ctor = Argument
                param.pop('help', None)
            param.pop('name', None)
            click_param = ctor(decls, **param)
            params[click_param.name] = Dict(
                decls=decls,
                **param,
            )
            click_params += [click_param]
        return click_params

    def add_otto_cmd(self, uid, task):
        task.name = task.get('name', uid)
        cmd = click.Group(
            task.name,
            chain=True,
            add_help_option=False,
            invoke_without_command=True,
            callback=self.callback(task),
        )
        cmd.params = self.add_params(task.params)
        return cmd

    def add_cmd(self, uid, task, cmd=None):
        task.name = task.get('name', uid)
        if cmd is None:
            cmd = click.Command(
                task.name,
                callback=self.callback(task),
            )
            cmd.params = []
        else:
            cmd.callback = self.callback(task)
        cmd.params = self.add_params(task.params)
        for subuid, subtask in task.tasks.items():
            subcmd = self.add_cmd(subuid, subtask)
            cmd.add_command(
                subcmd,
                name=subcmd.name,
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
        default_spec = self.otto_spec(otto_yml, otto_jobs, otto_version)
        cmd = self.add_otto_cmd(prog or 'otto', default_spec.otto)
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
        )
        cmd.add_help_option=True
        cmd.params = []
        otto_spec = otto_load(otto_yml=default_spec.otto.params.otto_yml.value)
        otto_spec.update(self.otto_spec(otto_yml, otto_jobs, otto_version))
        cmd = self.add_cmd(prog or 'otto', otto_spec.otto, cmd=cmd)
        cmd.main(
            args=self.remainder,
            standalone_mode=False,
        )
        return self.cmds, otto_spec
