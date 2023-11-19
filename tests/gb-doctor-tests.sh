#!/bin/bash
cd ..

#for NUM in 0{1..9} 10 11
for NUM in  11
do
  echo "Test $NUM"
  cargo run "test_roms/$NUM.gb" > "logs/$NUM-results.txt" off 2>"logs/$NUM-trace.txt"

  cd gameboy-doctor/
  python gameboy-doctor "../logs/$NUM-results.txt" cpu_instrs $NUM
  cd ..
done
