#include "vision/camera/CameraManager.hpp"

CameraManager::CameraManager(int deviceId)
    : deviceId_(deviceId) {}

bool CameraManager::open() {
    cap_.open(deviceId_);
    return cap_.isOpened();
}

bool CameraManager::isOpen() const {
    return cap_.isOpened();
}

bool CameraManager::grabFrame(cv::Mat& outFrame) {
    if (!cap_.isOpened()) return false;
    return cap_.read(outFrame);
}

void CameraManager::close() {
    if (cap_.isOpened()) cap_.release();
}

