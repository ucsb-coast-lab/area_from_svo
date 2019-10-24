#!/bin/bash

if [ ! -d build ]; then
  mkdir build;
fi

rm -r build/*;
cd build;
cmake ..;
make;
cp -p ZEDCountFrames ..;
cd ..;
