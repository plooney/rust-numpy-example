#! /usr/bin/env python
# -*- coding: utf-8 -*-

"""

"""
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='pyext',
      version='0.1.0.0',
      rust_extensions=[
          RustExtension('pyext.rust_binding',
                        'pyext-example/Cargo.toml', binding=Binding.RustCPython)],
      packages=['pyext'],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
