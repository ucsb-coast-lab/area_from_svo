## Surface Area Estimate From Individual .svo File Frames
#### UCSB Coastal Oceanography and Autonomous Systems Laboratory

__Build Instructions for Linux__
```
$ mkdir build
$ cd build
$ cmake ..; make
$ mv ZEDAreaFromSVO ..
$ cd ..
$ ./ZEDAreaFromSVO $filename.svo $frame_number
```
The built binary should be be located in the same directory as the `.svo` file. It's also important to maintain a consistent directory structure, since most of the input and output paths are hard-coded and parsed accordingly at the moment. Running the program in the manner above should write to stdout with an area in mm^2 of the filtered and target-assigned pixels according to the current algorithms for calculating areas from those filtered pixels.

In `main.cpp`, all functions that are called from the Rust library are marked with `_rs` for clarity.

__Requirements:__
- ZED-supported NVIDIA GPU
- NVIDIA CUDA 10.0 (*not 10.1*)
- ZED SDK >= v2.8.3
- CMake >= 2.6 (ExternalProject module required)
- Rust >= 1.37-stable and `cargo`

Tested on Pop!\_OS 18.04LTS (system76-driver is helpful for dealing with CUDA 10.0 dependency).
