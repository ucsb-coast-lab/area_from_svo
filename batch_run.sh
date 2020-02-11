#!/bin/bash

# Since this script uses GNU Parallel, please remember to cite it in any publication
# that gets produced using that data!

### DEFINE FUNCTIONS

function usage {
  echo "Usage: $0 [-v <filename.svo>] [-c <stereo_image_csv_directory_path>]" 1>&2;
  exit 1;
}

function build_rust() {
    # Build the Rust image processing software
    echo "- Building Rust image processing"
    cd process_stereo_image_csvs_rs;
    cargo build --release;
    cd ..;
}

function build_zed {
    echo '- Compiling the executables'
    # Build the ZEDAreaFromSVO executable
    ./build.sh; # This should result in the placing of the ZEDAreaFromSVO in the current directory

    # Change into the zed_count_frames directory, build the ZEDCountFrames, and move it into the main directory
    cd zed_count_frames;
    ./build.sh; # builds the ZEDCountFrames executable
    cp -p ZEDCountFrames ..;
    cd ..;

}

function write_stereo_image_csvs {
    # makes sure that the folder ZEDWriteStereoImageCSV writes the .csv files to exists
    # If it doesn't the files won't get written
    svo_prefix=$(echo $1 | cut -d'.' -f1)
    if [ ! -d stereo_image_csvs/$svo_prefix ]; then
      mkdir stereo_image_csvs/$svo_prefix;
    fi
    #echo ".svo prefix is: $svo_prefix"
    echo "- Accessing $1 to count frames"
    chmod u+x ZEDCountFrames
    total_frames=$(./ZEDCountFrames $1)
    echo "- The total number of frames in $1 is $total_frames"
    # Now we'll run our analysis in parallel.
    # Since we have limited GPU memory, can only run up to three frames at once,
    # so we need to run sequential parallel commands.

    # ***EDIT THIS TO CHANGE HOW OFTEN FRAMES ARE SAMPLED FROM THE .SVO FILE***
    for ((i=0; i<$total_frames;i=i+3))
    do
        #parallel echo ::: $i $((i+1)) $((i+2))

        #./ZEDWriteStereoImageCSV $1 $i
        parallel ./ZEDWriteStereoImageCSV $1 ::: $i $((i+1)) $((i+2))
    done
}

function batch_run_stereo_csvs {

    # makes sure that the folder ZEDWriteStereoImageCSV writes the .csv files to exists
    # If it doesn't the files won't get written
    svo_prefix=$(echo $1 | cut -d'/' -f2)
    if [ ! -d processed_images/$svo_prefix ]; then
      mkdir processed_images/$svo_prefix;
    fi
    echo "The directory is: $1"
    files=($1/*.csv)
    num="${#files[@]}"
    echo "The number of csv files in $1 is $num"

    parallel ./process_stereo_image_csvs_rs/target/release/process_stereo_image_csvs_rs ::: $1/*.csv >> results.csv

}



### SCRIPT ###

# Defines the number of available cores
echo "The number of available cores is: $(nproc)"

while getopts ":v:c:" o; do
    case "${o}" in
        v)
            v=${OPTARG}
            ;;
        c)
            c=${OPTARG}
            ;;
        *)
            usage
            ;;
    esac
done
shift $((OPTIND-1))

echo "v = ${v}"
echo "c = ${c}"

# If neither an .svo file nor csv directory are provied, usage()
if [ -z "${v}" ] && [ -z "${c}" ]; then
    usage
fi

# If only a .svo file is provided, build and parse .svo
if [ ! -z "${v}" ] && [ -z "${c}" ]; then
    build_zed
    echo "- Parsing .svo file!"
    write_stereo_image_csvs ${v}
fi

# If only a csv directory is provided, build and process csvs
if [ -z "${v}" ] && [ ! -z "${c}" ]; then
    build_rust
    echo '- Processing .csv files!'
    batch_run_stereo_csvs ${c}
    # Data visualization with Python
    ./plot_results.py
fi

# If both a csv directory and is provided, build and process csvs
if [ ! -z "${v}" ] && [ ! -z "${c}" ]; then
    build_zed
    build_rust
    echo 'WOULD BE BOTH PARSING .SVO FILE AND PROCESSING .CSV FILES NOW!'
    write_stereo_image_csvs ${v}
    batch_run_stereo_csvs ${c}
    # Data visualization with Python
    ./plot_results.py
fi
