#include <sl/Camera.hpp>
#include <string>
#include <fstream>
#include <iostream>
#include <chrono>
#include <thread>
// FFI include for Rust library
#include "process_stereo_image_csvs_rs.h"

using namespace sl;

int main(int argc, char **argv) {

    // Create ZED objects
    Camera zed;
    InitParameters initParameters;
    // Accepting the command line arguments for this program, where the .svo file
    // (which must be located in same directory as the binary!) is $1 and the frame
    // number we wish to access and analyze is $2
    std::string svo_filename = argv[1];
    int frame_num = std::atoi(argv[2]);
    std::string delimiter = ".svo";
    std::string token = svo_filename.substr(0, svo_filename.find(delimiter));
    std::string csv_filename = "stereo_image_csvs/" + token + "_" + std::to_string(frame_num) + ".csv";

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
    printf("The number of frames in this svo is: %i\n",total_frames);
    std::cout << "The requested svo frame is: " << frame_num << std::endl;
    std::cout << "This will translate to a csv file at: " << csv_filename << std::endl;

    int width = zed.getResolution().width;
    int height = zed.getResolution().height;
    printf("ZED (width, height) = (%i,%i)\n",width,height);

    sl::Mat image, depth, point_cloud;
    int svo_frame = zed.getSVOPosition();
    zed.setSVOPosition(frame_num);
    // Sleeps the thread for a set amount of milliseconds
    //std::this_thread::sleep_for(std::chrono::milliseconds(2000));

    // I literally have no idea why, but on svo playback, even if we use the setSVOPosition
    // function first, we'll only end up grabbing that frame on the THIRD time of calling
    // the zed.grab() function. I've checked that this isn't a timing issue or anything by
    // sleeping that thread, and that doesn't seem to have any effect. As a result, instead
    // of just calling grab() once in the if-statement calling it three times.
    // TO_DO: File a GH issue with ZED about this to find out what's going on

    if ((zed.grab() == SUCCESS) && (zed.grab() == SUCCESS) && (zed.grab() == SUCCESS)) {
            std::cout << "Accessing image" << std::endl;
            zed.retrieveImage(image, VIEW_LEFT); // Get the rectified left image
            std::string left_image_filename = "left_view_images/left" + std::to_string(frame_num) + ".png";
            //image.write(left_image_filename);
            image.write(left_image_filename.c_str());

            std::ofstream file;
            file.open(csv_filename);
            zed.retrieveMeasure(point_cloud, MEASURE_XYZRGBA);
            file << "pixel_x," << "pixel_y," << "x," << "y," << "z," << "R," << "G," << "B" << std::endl;
            std::cout << "About to process the imagery:" << std::endl;
            for (int i = 0; i < width; i++) {
                for (int j = 0; j < height; j++) {
                    sl::float4 point3D;
                    sl::uchar4 left_pixel;
                    point_cloud.getValue(i,j,&point3D);
                    float x = point3D.x;
                    float y = point3D.y;
                    float z = point3D.z;
                    image.getValue<sl::uchar4>(i, j, &left_pixel);
                    //std::cout << "leftImage center pixel R:" << (int)leftCenter.r << " G:" << (int)leftCenter.g << " B:" << (int)leftCenter.b << std::endl;

                    // If any of the point cloud values are invalid, set all the point values to zero
                    if (std::isfinite(x) == false || std::isfinite(y) == false || std::isfinite(z) == false ) {
                        x = 0;
                        y = 0;
                        z = 0;
                        // color = 0;
                        //std::cout << "The location of this point is invalid" << std::endl;
                    }
                    //std::cout << std::fixed << x << "," << y << "," << z << "," << int(left_pixel.r) << "," << int(left_pixel.g) << "," << int(left_pixel.b) << std::endl;
                    file << std::fixed << i << "," << j << "," << x << "," << y << "," << z << "," << int(left_pixel.r) << "," << int(left_pixel.g) << "," << int(left_pixel.b) << std::endl;
                }
            }
    }
    else if (zed.grab() != SUCCESS) {
        std::cout << "We had an error opening the camera view successfully" << std::endl;
    }

    // Close the camera
    zed.close();
    open_file_rs(csv_filename.c_str());
    print_area_rs(csv_filename.c_str());

    return 0;
}
