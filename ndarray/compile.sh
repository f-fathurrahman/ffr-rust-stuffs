#!/bin/bash
EXTLIBS=~/.cargo/extlibs
basname=`basename $1 .rs`
rustc $1 -L dependency=$EXTLIBS \
--extern ndarray=$EXTLIBS/libndarray-0.7.3.rlib \
--extern num_traits=$EXTLIBS/libnum_traits-0.1.32.rlib \
--extern num_complex=$EXTLIBS/libnum_complex-0.1.32.rlib \
-o "$basname.exe"
