#!/bin/bash
NUM=$1
NAME=$2
cargo run "gb-test-roms/cpu_instrs/individual/$NAME" > "$NUM-results.txt" 2>$NUM-trace.txt
cd gameboy-doctor/
python gameboy-doctor "../$NUM-results.txt" cpu_instrs $NUM
