#!/bin/bash
basname=`basename $1 .rs`
echo $basname
rustc "$1" -o "$basname.exe"

