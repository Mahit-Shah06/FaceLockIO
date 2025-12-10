#pragma once
#include <opencv2/opencv.hpp>
#include <opencv2/dnn.hpp>
#include <optional>
#include <string>
#include <vector>

struct DetectedFace {
    cv::Rect box;          // x, y, width, height
    float confidence;      // 0.0 → 1.0
    std::optional<cv::Mat> landmarks;  
};

enum class FaceState {
    NO_FACE,
    FACE_PRESENT,
    FACE_LOOKING_AWAY,
    EYES_CLOSED,
    MULTIPLE_FACES,
    SPOOF_RISK
};

class FaceDetector {
public:
    FaceDetector(const std::string& modelsBasePath);

    bool loadModels();  

    std::vector<DetectedFace> detect(const cv::Mat& frame);

    FaceState evaluateState(const std::vector<DetectedFace>& faces);

    bool shouldRunFast() const;
    bool shouldRunSlow() const;

private:
    // Primary detector (YuNet ONNX)
    cv::dnn::Net yunet;

    // Fallback detector (SSD Caffe)
    cv::dnn::Net ssd;

    int noFaceCounter = 0;
    int fastModeThreshold = 2;
    int slowModeThreshold = 6;

    bool fastMode = false;

    std::string yunetModelPath;
    std::string ssdProtoPath;
    std::string ssdModelPath;

    std::vector<DetectedFace> detectYuNet(const cv::Mat& frame);
    std::vector<DetectedFace> detectSSD(const cv::Mat& frame);

    void updateMode(bool faceDetected);
};

