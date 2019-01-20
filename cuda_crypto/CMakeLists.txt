# because cmake fully supports cuda until 3.8
cmake_minimum_required(VERSION 3.8)
project(cuda_crypto LANGUAGES CXX CUDA)
add_compile_options(-std=c++11)

set(CMAKE_CUDA_FLAGS "-arch=sm_35 -Xcompiler -fPIC")
set(CMAKE_CUDA_FLAGS_RELEASE "-O3 -DNDEBUG")

add_library(
    cuda_crypto STATIC # generate .a file
    ${PROJECT_SOURCE_DIR}/src/sha.h
    ${PROJECT_SOURCE_DIR}/src/md5.cu
    ${PROJECT_SOURCE_DIR}/src/sha256.cu
    ${PROJECT_SOURCE_DIR}/src/sha384.cu
    ${PROJECT_SOURCE_DIR}/src/sha512.cu
)

# set_target_properties(cuda_crypto PROPERTIES PREFIX "")
install(TARGETS cuda_crypto DESTINATION .)
