#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
import errno

sys.path.insert(0, os.path.dirname(__file__))
from doit.task import dict_to_task
from doit.doit_cmd import DoitMain
from doit.cmd_base import TaskLoader2

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

    def __repr__(self):
        return f'OttoTaskLoader(tasks={self.tasks})'

    def setup(self, opt_values):
        pass

    def load_doit_config(self):
        return {
            'verbosity': 2,
        }

    def load_actions(self, task):
        def create_param(param):
            value = param.value
            if isinstance(param.value, (tuple, list)):
                value = ' '.join(param.value)
            return f'{param.name}="{value}"'
        envs = ' '.join([create_param(param) for uid, param in task.params.items()]).strip()
        actions = [task.action] if 'action' in task else [] + task.get('actions', [])
        def create_action(i, action):
            taskpath = f'.otto/{task.name}'
            mkdir_p(taskpath)
            with open(f'{taskpath}/action{i}', 'w') as f:
                f.write(action)
            action = f'env {envs} bash {taskpath}/action{i}'
            return action
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
        if spec.otto.tasks:
            loader = OttoTaskLoader(spec.otto.tasks)
        else:
            loader = OttoTaskLoader({'tasks': spec.otto})
        if cmds:
            sys.exit(DoitMain(loader).run(cmds))
