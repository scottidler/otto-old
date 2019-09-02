#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from otto.parse import otto_parse
from otto.execute import otto_execute

from leatherman.dbg import dbg

def main():
    cmds, spec = otto_parse()
    otto_execute(cmds, spec)

if __name__ == '__main__':
    main()
