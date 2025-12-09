cmake_minimum_required(VERSION 3.16)

project(FaceLockIO LANGUAGES CXX)

# Make your include/ folder visible to the compiler
include_directories(include)

find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add source folders
add_subdirectory(src)

# Output directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

