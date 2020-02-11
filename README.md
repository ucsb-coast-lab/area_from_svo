## Surface Area Estimate From ZED `.svo` Video Files
#### UCSB Coastal Oceanography and Autonomous Systems Laboratory

The repository contains the parts necessary to do analysis on ZED stereo .svo files. This is primarily done by using the ZED API to access the x,y,z point cloud information for each pixel in the image and mapping that to the approximate location of the corresponding pixel in the left view. These values are then written, along with the pixel's x,y image location and RGB color information into an ASCII `.csv` file, which can then be post-processed (done here using Rust). Since this is an inexact measurement method, most usable information should be generated as aggregates, requiring batch processing of the `.svo` video files frames. As a result, parallelized operations for both writing and processing the stereo image `.csv` files is important to keep analysis times manageable.

__Build Instructions for Linux__

The `batch_run.sh` script has an option to accept any combination of two options: `-v` for processing a `.svo` video file, `-c` for specifying the directory of the stereo image .csv files that should be processed.

```
# Only write the frames from the .svo to file
$ ./batch_run.sh -v example.svo

# Only process the .csv files in the specified directory
$ ./batch_run.sh -c stereo_image_csvs/example

# Write the stereo image to file and process from the directory path
$ ./batch_run.sh -v example.svo -c stereo_image_csvs/example
```
The frame sub-sampling rate and image processing method (for example, an image processing script in Python or MATLAB) can be specified by modifying the `batch_run.sh` script. That script is designed to follow the directory structure laid out in this README document

__Build Requirements__

Make sure to follow the appropriate build instructions for ZED and OpenCV, and be sure to add the relevant paths to the `.bashrc` file

For writing .csv files:
- ZED-supported NVIDIA GPU
- NVIDIA CUDA 10.0 (*not 10.1*)
- ZED SDK = v2.8.3 or v2.8.5
- CMake >= 2.6 (ExternalProject module required)
- GNU Parallel (batch_run only, please remember to cite in publications)
- OpenCV 3.4

For stereo image .csv processing
- Rust >= 1.37-stable and `cargo`
- GNU Parallel (batch_run only, please remember to cite in publications)

For data visualization:
- Python 3.7 and Numpy


__Recommended Directory Structure__
```
.
├── angle_calculations
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── batch_run.sh
├── build
├── build.sh
├── CMakeLists.txt
├── left_view_images
├── main.cpp
├── plot_results.py
├── processed_images
│   └── pool
│       └── pool_0.png
├── process_stereo_image_csvs_rs
│   ├── binding
│   │   └── process_stereo_image_csvs_rs.h
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   └── tests.rs
│   └── target
├── README.md
├── stereo_image_csvs
│   ├── pool
│   │   └── pool_0.csv
│   └── placeholder.md
├── results.csv
├── videofile.svo
└── zed_count_frames
    ├── build.sh
    ├── CMakeLists.txt
    └── main.cpp


```

__Requirements:__
- ZED-supported NVIDIA GPU
- NVIDIA CUDA 10.0 (*not 10.1*)
- ZED SDK >= v2.8.3
- CMake >= 2.6 (ExternalProject module required)
- Rust >= 1.37-stable and `cargo`

Tested on Pop!\_OS 18.04LTS and Ubuntu 18.04LTS (`system76-driver` is helpful for dealing with CUDA 10.0 dependency).

If using this program to process data for publication, please cite the UCSB COAST Lab. Thank you!
