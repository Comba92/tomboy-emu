#!/bin/bash

for NUM in $1 $2
do
  echo "Test $NUM"
  cargo run -q "test_roms/$NUM.gb" $3 > "logs/$NUM-results.log" 2>"logs/$NUM-trace.log"

  # python gameboy-doctor "../logs/$NUM-results.log" cpu_instrs $NUM
done