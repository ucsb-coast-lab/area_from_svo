# UCSB COAST Lab ZED .svo parsing

This should produce a binary that will parse a single frame of a ZED .svo file, save that image, and produce a .csv file with it x,y pixel location, depth cloud location, and color information. This can then be moved into the `read_array` directory, which parses that .csv file. It will reconstruct the left-side  view (same image saved by the C++ binary), and can be modified such that the depth and color information can be used to generate filters. 
