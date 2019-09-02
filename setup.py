#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup, find_packages

with open('README.md') as f:
    long_description = f.read()

with open('requirements.txt') as f:
    app_requirements = f.read().splitlines()

with open('tests/requirements.txt') as f:
    test_requirements = f.read().splitlines()

setup_requirements = [
    'pytest-runner',
    'setuptools>=40.5.0'
]

extras = {
    'test': test_requirements
}

setup(
    name='otto',
    version='v0.1',
    author='Scott Idler',
    author_email='scott.a.idler@mozilla.com',
    description='dependency based cli interface with yaml support',
    long_description=long_description,
    url='https://github.com/scottidler/otto',
    classifiers=(
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT',
        'Operating System :: OS Independent',
    ),
    license='MIT',
    include_package_data=True,
    packages=find_packages(include=['otto']),
    entry_points='''
    [console_scripts]
    otto=otto.cli:main
    ''',
    install_requires=app_requirements,
    setup_requires=setup_requirements,
    tests_require=test_requirements,
    test_suite='tests',
    extras_require=extras,
    zip_safe=False,
)
