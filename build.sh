#!/bin/bash

rm -r build/*;
cd build;
cmake ..;
make;
cp -p ZEDAreaFromSVO ..;
cd ..;
