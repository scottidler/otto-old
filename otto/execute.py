#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
import errno

sys.path.insert(0, os.path.dirname(__file__))
from doit.task import dict_to_task
from doit.doit_cmd import DoitMain
from doit.cmd_base import TaskLoader2

from otto.parse import otto_parse

from leatherman.dbg import dbg

def otto_execute(cmds, spec):
    executor = OttoExecutor(cmds, spec)
    executor.execute()

def mkdir_p(path):
    try:
        os.makedirs(path)
    except OSError as ex:  # Python >2.5
        if ex.errno == errno.EEXIST and os.path.isdir(path):
            pass
        else:
            raise

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
        actions = [task.action] if 'action' in task else [] + task.get('actions', [])
        dbg(params=task.params)
        envs = ''.join([f'{param.name}={param.value} ' for uid, param in task.params.items()]).strip()
        def create_action(i, action):
            taskpath = f'.otto/{task.name}'
            mkdir_p(taskpath)
            with open(f'{taskpath}/action{i}', 'w') as f:
                f.write(action)
            return f'{envs} bash {taskpath}/action{i}'
        return [
            create_action(i, action) for i, action in enumerate(actions)
        ]

    def load_tasks(self, cmd, pos_args):
        return [
            dict_to_task(dict(
                name=task.get('name', uid),
                task_dep=task.get('deps', []),
                actions=self.load_actions(task),
                uptodate=task.get('uptodate', []),
            )) for uid, task in self.tasks.items()
        ]

class OttoExecutor:
    def __init__(self, cmds, spec):
        self.cmds = cmds
        self.spec = spec

    def execute(self, cmds=None, spec=None):
        cmds = cmds or self.cmds
        spec = spec or self.spec
        loader = OttoTaskLoader(spec.otto.tasks)
        if len(cmds) > 1:
            sys.exit(DoitMain(loader).run(cmds))
