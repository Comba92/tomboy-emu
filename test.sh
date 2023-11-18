#!/bin/bash
NUM=$1
cargo run "test_roms/$NUM.gb" > "logs/$NUM-results.txt" off 2>"logs/$NUM-trace.txt"

cd gameboy-doctor/
python gameboy-doctor "../logs/$NUM-results.txt" cpu_instrs $NUM
