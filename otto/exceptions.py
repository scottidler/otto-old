#!/usr/bin/env python3
# -*- coding: utf-8 -*-

class OttoYmlLoadError(Exception):
    def __init__(self, filename):
        msg = f'error when loading otto yml file: {filename}'
        super().__init__(msg)
