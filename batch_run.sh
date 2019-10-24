#!/bin/bash

# Since this script uses GNU Parallel, please remember to cite it in any publication
# that gets produced using that data!

# Define the ZED .svo file that we want to process
svo_file='pool.svo';

# First, we're going make both the ZEDAreaFromSVO and ZEDCountFrames executables
./build.sh; # This should result in the placing of the ZEDAreaFromSVO in the current directory
cd zed_count_frames;
./build.sh; # builds the ZEDCountFrames executable
cp -p ZEDCountFrames ..;
cd ..;
# Both binaries should now be in the current directory

# Now going to assign the return value of the ZEDCountFrames binary to a variable
total_frames=$(./ZEDCountFrames $svo_file)
echo $total_frames

# Now we'll run our analysis in parallel.
# Since we have limited GPU memory, can only run up to three frames at once,
# so we need to run sequential parallel commands.
# We can modify this for-loop to specify the range of frames that we're interested in too
for ((i=2151; i<2154;i=i+3))
do
    # parallel echo ::: $i $((i+1)) $((i+2))
    parallel ./ZEDAreaFromSVO $svo_file ::: $i $((i+1)) $((i+2))
done

# Processing frame numbers for serial1: 638 913 1133 1419 1945 2229
#                              serial2: 545 805 1085 1098 1417 1520 1727
