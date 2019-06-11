#!/usr/bin/env python3
# -*- coding: utf-8 -*-

DOIT_CONFIG = {
    'verbosity': 2,
    'default_tasks': [
        'black',
    ],
}

def task_black():
    '''
    run doit black -S otto/
    '''
    return {
        'actions': [
            'black -S otto/',
        ],
        'uptodate': [
            'black -S --check otto/',
        ],
    }
