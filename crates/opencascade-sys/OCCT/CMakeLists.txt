cmake_minimum_required (VERSION 3.1 FATAL_ERROR)
project (OpenCASCADEPackageConfig)

# Fix for WASM: https://github.com/emscripten-core/emscripten/issues/19243
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE "BOTH" CACHE PATH "")

find_package (OpenCASCADE REQUIRED)

file (WRITE ${CMAKE_BINARY_DIR}/occ_info.txt
      "VERSION_MAJOR=${OpenCASCADE_MAJOR_VERSION}\n"
      "VERSION_MINOR=${OpenCASCADE_MINOR_VERSION}\n"
      "INCLUDE_DIR=${OpenCASCADE_INCLUDE_DIR}\n"
      "LIBRARY_DIR=${OpenCASCADE_LIBRARY_DIR}\n"
      "BUILD_SHARED_LIBS=${OpenCASCADE_BUILD_SHARED_LIBS}\n")

install (FILES ${CMAKE_BINARY_DIR}/occ_info.txt TYPE DATA)
