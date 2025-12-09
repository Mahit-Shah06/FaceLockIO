#include <iostream>
#include <opencv2/opencv.hpp>
#include "vision/camera/CameraManager.hpp"

int main() {
    CameraManager cam(0);

    if (!cam.open()) {
        std::cout << "ERROR: Failed to open camera\n";
        return 1;
    }

    std::cout << "Camera opened successfully.\n";

    cv::Mat frame;
    if (cam.grabFrame(frame)) {
        std::cout << "Frame captured: " 
                  << frame.cols << "x" 
                  << frame.rows << "\n";
    } else {
        std::cout << "Failed to capture frame.\n";
    }

    cam.close();
    return 0;
}

