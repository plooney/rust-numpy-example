#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2017 zenbook <zenbook@zenbook-XPS>
#
# Distributed under terms of the MIT license.

"""

"""
#!/usr/bin/env python

from pyext import rust_binding
import numpy as np
import faster_numpy as fn
from benchmarker import Benchmarker

if __name__ == '__main__':
    a = np.arange(1000.0)
    assert(np.sum(a) == rust_binding.sum(a))
    with Benchmarker(100000, width=50) as bench:
        @bench("numpy.sum")
        def _(bm):
            for i in bm:
                np.sum(a)

        @bench("faster_numpy.clibrary.sum")
        def _(bm):
            for i in bm:
                fn.clibrary.sum(a)

        @bench("rust_binding.sum")
        def _(bm):
            for i in bm:
                rust_binding.sum(a)
