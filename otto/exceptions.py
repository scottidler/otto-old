#!/usr/bin/env python3
# -*- coding: utf-8 -*-


class OttoYmlLoadError(Exception):
    def __init__(self, filename):
        msg = f'error when loading otto yml file: {filename}'
        super().__init__(msg)


class ArgSpecError(Exception):
    def __init__(self, arg):
        msg = f'arg spec error for arg="{arg}"'
        super().__init__(msg)
