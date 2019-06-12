#!/usr/bin/env python3
# -*- coding: utf-8 -*-

def head_body(d):
    assert len(d) == 1
    return d.items()

def head(d):
    assert len(d) == 1
    return d.keys()[0]

def body(d):
    assert len(d) == 1
    return d.values()[0]
