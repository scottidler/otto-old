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
from otto.load import otto_load, OttoLoader

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

    def callback(self, task):
        def callback(*args, **kwargs):
            self.remainder = kwargs.pop('remainder', [])
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
        })

    def add_params(self, params):
        click_params = []
        new_params = {}
        for uid, param in deepcopy(params).items():
            assert isinstance(uid, str) and uid != ''
            kwargs = dict(**param)
            if '-' in uid:
                decls = tuple(uid.split('|'))
                ctor = Option
            else:
                decls = (uid,)
                ctor = Argument
                kwargs.pop('help', None)
            click_param = ctor(decls, **kwargs)
            new_params[click_param.name] = Dict(
                decls=decls,
                **kwargs,
            )
            click_params += [click_param]
            params.pop(uid)
        params.update(new_params)
        return click_params

    def add_otto_cmd(self, spec):
        cmd = click.Group(
            spec.otto,
            chain=True,
            add_help_option=False,
            invoke_without_command=True,
            callback=self.callback(spec.otto),
        )
        cmd.params = self.add_params(spec.otto.params)
        return cmd

    def add_cmd(self, spec, cmd=None):
        if cmd is None:
            cmd = click.Command(
                spec.otto,
                callback=self.callback(spec.otto),
            )
            cmd.params = []
        else:
            cmd.callback = self.callback(spec.otto)
        cmd.params = self.add_params(spec.otto.params)
        for subuid, subtask in spec.otto.tasks.items():
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
        otto_spec1 = self.otto_spec(otto_yml, otto_jobs, otto_version)
        cmd = self.add_otto_cmd(otto_spec1)
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
            obj={},
        )
        cmd.add_help_option=True
        #cmd.params = cmd.params[:-1]
        cmd.params = []
        user_spec = otto_load(otto_yml=otto_spec1.otto.params.otto_yml.value)
        user_spec.update(self.otto_spec(otto_yml, otto_jobs, otto_version))
        cmd = self.add_cmd(user_spec, cmd=cmd)
        cmd.main(
            args=self.remainder,
            standalone_mode=False,
            obj={},
        )
        dbg(user_spec)
