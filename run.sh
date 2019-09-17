#!/bin/bash

cd build;
cmake ..;
make;
mv ZEDDepthAndColor ..
cd ..
#eog depth_test.png
