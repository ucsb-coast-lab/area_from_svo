#include <sl/Camera.hpp>
#include <string>
#include <fstream>
#include <iostream>

using namespace sl;

int main(int argc, char **argv) {

    // Create ZED objects
    Camera zed;
    InitParameters initParameters;
    // Accepting the command line arguments for this program, where the .svo file
    // (which must be located in same directory as the binary!) is $1 and the frame
    // number we wish to access and analyze is $2
    std::string svo_filename = argv[1];

    initParameters.input.setFromSVOFile(argv[1]);
    RuntimeParameters runtime_param;
    runtime_param.sensing_mode = SENSING_MODE_STANDARD;
	initParameters.depth_mode = DEPTH_MODE_ULTRA;

    // Open the ZED
    ERROR_CODE err = zed.open(initParameters);
    if (err != SUCCESS) {
        std::cout << toString(err) << std::endl;
        zed.close();
        return 1; // Quit if an error occurred
    }

    int total_frames = zed.getSVONumberOfFrames();
    // Close the camera
    zed.close();

    // Returns the number of frames in the specified .svo file, which should be captured in a bash script
    // std::cout << "The total number of frames in " << svo_filename << " is " << total_frames << std::endl;
    // Since this is the last thing to be written out, we'll capture this output in a bash script and use a command substitution
    std::cout << total_frames << std::endl;
}
