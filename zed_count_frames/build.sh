#!/bin/bash

rm -r build/;
mkdir build;
cd build;
cmake ..;
make;
cp -p ZEDCountFrames ../..; # Moves to main directory
cd ..;
