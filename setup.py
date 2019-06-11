#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup, find_packages

with open("README.md", "r") as f:
    long_description = f.read()

requirements = [
    'attrdict',
    'ruamel.yaml',
]

setup_requirements = [
    'pytest-runner',
    'setuptools>=40.5.0',
]

test_requirements = [
    'pytest',
    'pytest-cov',
    'pytest-mock',
    'pytest-watch',
]

extras = {'test': test_requirements}

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
    install_requires=requirements,
    license='MIT',
    include_package_data=True,
    packages=find_packages(include=['otto']),
    setup_requires=setup_requirements,
    test_suite='tests',
    tests_require=test_requirements,
    extras_require=extras,
    zip_safe=False,
)
