SET(execName ZEDWriteStereoImageCSV)
CMAKE_MINIMUM_REQUIRED(VERSION 2.6)
PROJECT(${execName})

option(LINK_SHARED_ZED "Link with the ZED SDK shared executable" ON)

if (NOT LINK_SHARED_ZED AND MSVC)
    message(FATAL_ERROR "LINK_SHARED_ZED OFF : ZED SDK static libraries not available on Windows")
endif()

if(COMMAND cmake_policy)
	cmake_policy(SET CMP0003 OLD)
	cmake_policy(SET CMP0015 OLD)
endif(COMMAND cmake_policy)

find_package(ZED 2 REQUIRED)
find_package(CUDA ${ZED_CUDA_VERSION} EXACT REQUIRED)

include_directories(${CUDA_INCLUDE_DIRS})
include_directories(${ZED_INCLUDE_DIRS})
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(~/opencv/include)

link_directories(${ZED_LIBRARY_DIR})
link_directories(${CUDA_LIBRARY_DIRS})
# Defines the locations of the ZED_LIBS variable called in target_link_libraries()
if (LINK_SHARED_ZED)
    SET(ZED_LIBS ${ZED_LIBRARIES} ${CUDA_CUDA_LIBRARY} ${CUDA_CUDART_LIBRARY} ${CUDA_NPP_LIBRARIES_ZED})
else()
    SET(ZED_LIBS ${ZED_STATIC_LIBRARIES} ${CUDA_CUDA_LIBRARY} ${CUDA_LIBRARY})
endif()

SET(EXECUTABLE_OUTPUT_PATH ".")

##################
### RUST INCLUDES
##################

if(FALSE)

SET(RUST_LIB process_stereo_image_csvs_rs)
include_directories ("${PROJECT_BINARY_DIR}")
include_directories ("${PROJECT_SOURCE_DIR}/${RUST_LIB}/binding")
include (ExternalProject)
set_directory_properties (PROPERTIES EP_PREFIX ${PROJECT_BINARY_DIR}/RUST)

ExternalProject_Add (
  ${RUST_LIB}
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  BUILD_COMMAND "" cargo build COMMAND cargo build --release COMMAND cbindgen -o binding/process_stereo_image_csvs_rs.h
  BINARY_DIR "${PROJECT_SOURCE_DIR}/${RUST_LIB}"
  INSTALL_COMMAND ""
  LOG_BUILD ON
 )

 endif()
#######################

# ADD_EXECUTABLE(${execName} "main.cpp")
SET(SRC
  main.cpp
)

ADD_EXECUTABLE(${execName} ${SRC})
add_definitions(-std=c++11 -g -O3)

TARGET_LINK_LIBRARIES(
    ${execName}
    ${ZED_LIBS}
    # debug "${PROJECT_SOURCE_DIR}/${RUST_LIB}/target/debug/libprocess_stereo_image_csvs_rs.a"
    # optimized "${PROJECT_SOURCE_DIR}/${RUST_LIB}/target/release/libprocess_stereo_image_csvs_rs.a"
    m
    dl
    pthread
)
