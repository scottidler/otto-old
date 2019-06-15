#!/usr/bin/env python3
# -*- coding: utf-8 -*-


class OttoArgs:
    def __init__(
        self,
        name,
        action=None,
        nargs=None,
        const=None,
        default=None,
        type=None,
        choices=None,
        required=None,
        help=None,
    ):
        pass


class OttoTask:
    def __init__(self, name, actions, deps=None, help=None, args=None, tasks=None):
        self.name = name
        self.args = args
        self.tasks = tasks
