#!/bin/bash

if [ ! -d left_view_images ]; then
  mkdir left_view_images;
fi

if [ ! -d processed_images ]; then
  mkdir processed_images;
fi

if [ ! -d build ]; then
  mkdir build;
fi

# If the build folder doesn't exist, make it
if [ ! -d build ]; then
  mkdir build;
fi



rm -r build/*;
cd build;
cmake ..;
make;
# cp -p ZEDAreaFromSVO ..;
cp -p ZEDWriteStereoImageCSV ..;
cd ..;
