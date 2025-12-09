#pragma once
#include <opencv2/opencv.hpp>

class CameraManager {
public:
    CameraManager(int deviceId = 0);

    bool open();
    bool isOpen() const;
    bool grabFrame(cv::Mat& outFrame);
    void close();

private:
    int deviceId_;
    cv::VideoCapture cap_;
};

