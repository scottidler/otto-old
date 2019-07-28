#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
sys.path.insert(0, os.path.dirname(__file__))
from doit.task import dict_to_task
from doit.doit_cmd import DoitMain
from doit.cmd_base import TaskLoader2

from otto.parse import otto_parse

from leatherman.dbg import dbg

def otto_execute(cmds, spec):
    executor = OttoExecutor(cmds, spec)
    executor.execute()


class OttoTaskLoader(TaskLoader2):
    def __init__(self, tasks):
        self.tasks = tasks

    def setup(self, opt_values):
        pass

    def load_doit_config(self):
        return {
            'verbosity': 2,
        }

    def load_actions(self, task):
        return [task.action] if 'action' in task else [] + task.get('actions', [])

    def load_tasks(self, cmd, pos_args):
        return [
            dict(
                name=task.get('name', uid),
                task_dep=task.get('deps', []),
                actions=self.load_actions(task),
                uptodate=task.get('uptodate', []),
            ) for uid, task in self.tasks.items()
        ]

class OttoExecutor:
    def __init__(self, cmds, spec):
        self.cmds = cmds
        self.spec = spec

    def execute(self, cmds=None, spec=None):
        cmds_ = cmds or self.cmds
        spec_ = spec or self.spec
        loader = OttoTaskLoader(spec_.otto.tasks)
        if len(cmds_) > 1:
            dbg(cmds_)
            sys.exit(DoitMain(loader).run(cmds_))
