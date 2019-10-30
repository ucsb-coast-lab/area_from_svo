#!/bin/bash

# Since this script uses GNU Parallel, please remember to cite it in any publication
# that gets produced using that data!


### DEFINE FUNCTIONS
say_hello() {
    echo $1
}

run_build_scripts() {
    echo '- Compiling the executables'
    # Build the ZEDAreaFromSVO executable
    ./build.sh; # This should result in the placing of the ZEDAreaFromSVO in the current directory

    # Change into the zed_count_frames directory, build the ZEDCountFrames, and move it into the main directory
    cd zed_count_frames;
    ./build.sh; # builds the ZEDCountFrames executable
    cp -p ZEDCountFrames ..;
    cd ..;

    # Build the Rust image processing software
    echo "- Building Rust image processing"
    cd process_stereo_image_csvs_rs;
    cargo build --release;
    cd ..;
    # Both binaries should now be in the current directory
}

function write_stereo_image_csvs {
    # total_frames=$(./ZEDCountFrames $svo_file)
    total_frames=$(./ZEDCountFrames $1)
    echo $total_frames

    # Now we'll run our analysis in parallel.
    # Since we have limited GPU memory, can only run up to three frames at once,
    # so we need to run sequential parallel commands.
    # We can modify this for-loop to specify the range of frames that we're interested in too
    for ((i=0; i<$total_frames;i=i+3))
    do
        # parallel echo ::: $i $((i+1)) $((i+2))
        parallel ./ZEDAreaFromSVO $svo_file ::: $i $((i+1)) $((i+2)) >> results.csv
    done
}
# Processing frame numbers for serial1: 638 913 1133 1419 1945 2229
#                              serial2: 545 805 1085 1098 1417 1520 1727

function batch_run_stereo_csvs {
    echo 'Hello, world'
    csv_dir='stereo_image_csvs'
    files=($csv_dir/serial2*.csv);
    # num=$(ls -l | grep ^- | wc -l)
    num="${#files[@]}"
    echo "The number of csv files in $csv_dir is $num"

    for ((i=0;i<$num;i=i+8))
    do
        #./process_stereo_image_csvs_rs/target/release/process_stereo_image_csvs_rs ${files[$i]} # >> ../csv_only_results.csv
        parallel ./process_stereo_image_csvs_rs/target/release/process_stereo_image_csvs_rs ::: ${files[$i]} ${files[$((i+1))]} ${files[$((i+2))]} ${files[$((i+3))]} ${files[$((i+4))]} ${files[$((i+5))]} ${files[$((i+6))]} ${files[$((i+7))]} >> results.csv
        #echo "Done with frame $i"
    done

}

### SCRIPT ###

# Define the ZED .svo file that we want to process
svo_file=$1;
say_hello $svo_file;
run_build_scripts
# write_stereo_image_csvs $svo_file
batch_run_stereo_csvs
