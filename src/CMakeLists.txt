# Collect all .cpp files in module folders
file(GLOB_RECURSE CORE_SRC core/*.cpp)
list(REMOVE_ITEM CORE_SRC ${CMAKE_CURRENT_SOURCE_DIR}/core/camera_test.cpp)
list(REMOVE_ITEM CORE_SRC ${CMAKE_CURRENT_SOURCE_DIR}/core/camera_preview.cpp)
file(GLOB_RECURSE VISION_cam_SRC vision/camera/*.cpp)
file(GLOB_RECURSE VISION_SRC vision/*.cpp)
file(GLOB_RECURSE AUTH_SRC auth/*.cpp)
file(GLOB_RECURSE INPUT_SRC input/*.cpp)

add_executable(FaceLockIO
    ${CORE_SRC}
    ${VISION_cam_SRC}
    ${VISION_SRC}
    ${AUTH_SRC}
    ${INPUT_SRC}
)

add_executable(Camera_test
	core/camera_test.cpp
	${VISION_cam_SRC}
)

add_executable(Camera_preview
	core/camera_preview.cpp
	${VISION_cam_SRC}
)
target_link_libraries(FaceLockIO ${OpenCV_LIBS})
target_link_libraries(Camera_test ${OpenCV_LIBS})
target_link_libraries(Camera_preview ${OpenCV_LIBS})

