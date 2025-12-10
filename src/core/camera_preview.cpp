#include <opencv2/opencv.hpp>
#include <iostream>

using namespace cv;

int main(){
	cv::VideoCapture cap(0);
	if (!cap.isOpened()){
		std::cout << "Error opening camera\n";
		return -1;
	}

	while(true){
		cv::Mat frame;
		cap.read(frame);

		if (frame.empty()){
			break;
		}
		
		cv::imshow("Camera preview", frame);

		if (cv::waitKey(1) == 'q'){
			break;
		}
	}
	
}
