///////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2017, STEREOLABS.
//
// All rights reserved.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
///////////////////////////////////////////////////////////////////////////


#include <sl/Camera.hpp>
#include <string>
#include <fstream>
#include <iostream>
#include <chrono>
#include <thread>


// Sample includes// Sample includes
//#include <opencv2/opencv.hpp>
//#include "utils.hpp"

//using namespace std;
using namespace sl;

int main(int argc, char **argv) {

    // Create ZED objects
    Camera zed;
    InitParameters initParameters;
    // initParameters.svo_input_filename.set(argv[1]);
    initParameters.input.setFromSVOFile(argv[1]);
    RuntimeParameters runtime_param;
    runtime_param.sensing_mode = SENSING_MODE_STANDARD;


    int svo_number = std::atoi(argv[2]);
    std::cout << "The requested svo frame is: " << svo_number << std::endl;
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
    int width = zed.getResolution().width;
    int height = zed.getResolution().height;
    printf("ZED (width, height) = (%i,%i)\n",width,height);

    sl::Mat image, depth, point_cloud;
    int svo_frame = zed.getSVOPosition();
    zed.setSVOPosition(svo_number);
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
            image.write("left.png");

            std::string filename = "test.csv";
            std::ofstream file;
            file.open(filename);
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


    // Opens a file for logging the data
    /*
    std::string filename = "test.csv";
    std::ofstream file;
    file.open(filename);
    int k = 0;
    // while (zed.getSVOPosition() < zed.getSVONumberOfFrames()-1) {
    while (zed.getSVOPosition() <= svo_number) {

        if (zed.grab() == SUCCESS) {
            std::cout << "We're on iteration " << k << " at svo position " << zed.getSVOPosition() << std::endl;
            //std::cout << "At i = " << zed.getSVOPosition() << std::endl;
            zed.setSVOPosition(svo_number);
            zed.retrieveImage(image, VIEW_LEFT);
            // Writes the left camera view to a png file
            image.write("left.png");
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

    }
    */

    // Close the camera
    zed.close();
    return 0;
}
