#!/bin/bash
rustc test_ndarray_1.rs -L dependency=~/.cargo/extlibs \
--extern ndarray=~/.cargo/extlibs/libndarray-0.7.3.rlib \
--extern num_traits=~/.cargo/extlibs/libnum_traits-0.1.32.rlib \
--extern num_complex=~/.cargo/extlibs/libnum_complex-0.1.32.rlib
