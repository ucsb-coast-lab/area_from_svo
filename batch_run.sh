#!/bin/bash

# Since this script uses GNU Parallel, please remember to cite it in any publication
# that gets produced using that data!

./build.sh;
svo_file='serial2.svo';
# Processing frame numbers for serial1: 638 913 1133 1419 1945 2229
#                              serial2: 545 805 1085 1098 1417 1520 1727
# Since we have limited GPU memory, can only run up to three frames at once,
# so we need to run sequential parallel commands


parallel ./ZEDAreaFromSVO $svo_file ::: 545 805 1085;
echo "\n\t- Moving onto the next set of images: "
parallel ./ZEDAreaFromSVO $svo_file ::: 1098 1417 1520;
./ZEDAreaFromSVO $svo_file 1727;
