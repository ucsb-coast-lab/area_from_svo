## Surface Area Estimate From Individual .svo File Frames
#### UCSB Coastal Oceanography and Autonomous Systems Laboratory

The repository contains both a stand-alone program for analyzing a single frame from a ZED `.svo` video file, as well as a script to batch-run that analysis over an entire video file. This requires building a secondary program which return the total number of frames in that .svo file. Individual build scripts have been included for both of those repositories, which are then called by the `batch_run` script.

__Build Instructions for Linux__

To process a single .svo file:

```
$ ./build.sh
$ ./ZEDAreaFromSVO $filename.svo $frame_number
```
To batch run an entire .svo file:
```
$ ./batch_run.sh $filename.svo
```
__Build Requirements__
- ZED-supported NVIDIA GPU
- NVIDIA CUDA 10.0 (*not 10.1*)
- ZED SDK >= v2.8.3
- CMake >= 2.6 (ExternalProject module required)
- Rust >= 1.37-stable and `cargo`
- GNU Parallel (batch_run only, please remember to cite in publications)


Build scripts need to be run in the repositories of their respective repositories, and the `.svo` video file needs to be placed in the root of the directory. It's important to maintain a consistent directory structure, since most of the input and output paths are hard-coded and parsed accordingly at the moment. Running the program in the manner above should write to stdout with an area in mm^2 of the filtered and target-assigned pixels according to the current algorithms for calculating areas from those filtered pixels.

```
.
├── angle_calculations
│   ├── Cargo.toml
│   └── src
├── batch_run.sh
├── build.sh
├── CMakeLists.txt
├── main.cpp
├── process_stereo_image_csvs_rs
│   ├── binding
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── README.md
├── video_file.svo
└── zed_count_frames
    ├── build.sh
    ├── CMakeLists.txt
    └── main.cpp

```

In `main.cpp`, all functions that are called from the Rust library are marked with `_rs` for clarity.

__Requirements:__
- ZED-supported NVIDIA GPU
- NVIDIA CUDA 10.0 (*not 10.1*)
- ZED SDK >= v2.8.3
- CMake >= 2.6 (ExternalProject module required)
- Rust >= 1.37-stable and `cargo`

Tested on Pop!\_OS 18.04LTS (`system76-driver` is helpful for dealing with CUDA 10.0 dependency).

If using this program to process data for publication, please cite the UCSB COAST Lab. Thank you!
