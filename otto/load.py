#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

from ruamel import yaml
from addict import Dict

from otto.constants import *
from otto.exceptions import OttoYmlLoadError
from otto.task import OttoParam, OttoTask

from leatherman.repr import __repr__
from leatherman.dbg import dbg

def otto_load(otto_yml=OTTO_YML):
    loader = OttoLoader(otto_yml)
    return loader.load()


class OttoLoader:
    def __init__(self, otto_yml=None):
        self.otto_yml = otto_yml or OTTO_YML

    __repr__ = __repr__

    def load(self, otto_yml=None):
        spec = OttoLoader.load_spec(otto_yml or self.otto_yml)
        return spec

    @staticmethod
    def load_spec(otto_yml):
        try:
            spec = Dict(yaml.safe_load(open(otto_yml)))
        except Exception as ex:
            raise OttoYmlLoadError(otto_yml) from ex
        return spec
