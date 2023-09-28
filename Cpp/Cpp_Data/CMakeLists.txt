cmake_minimum_required(VERSION 3.25)
project(CppData)
#---------------------------------------------------
set(CMAKE_BUILD_TYPE Debug)
#---------------------------------------------------
#   CppData dir
#   -- CMakeLists.txt (this file)
#   -- src dir
#      -- Bits_Data.cpp
#      -- Bits_DataAnalysis.h
#      -- Bits_DataAnalysis.cpp
#   -- build directory
#      -- Debug directory 
#         -- Cpp_Data.exe 
#         -- ...
#---------------------------------------------------

# Wasn't able to get std::library modules to work with CMake.
# - does work in Visual Studio, preview edition, non CMake project
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}
  /experimental:module /std:c++latest /EHsc /MD"
)
set(CMAKE_MODULE_PATH "C:/Users/Public/std_modules")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++20")
# set_property(TARGET $CppData PROPERTY CXX_STANDARD 20)
# target_compile_features(CppData PUBLIC CXX_STANDARD 20)
# set(CMAKE_CXX_FLAGS "${/experimental:module /std:c++latest}")
# set(CMake_CXX_STANDARD 20)
# set(CMAKE_CXX_STANDARD_REQUIRED ON)
# set(CMAKE_CXX_EXTENSIONS OFF)

#---------------------------------------------------
# build Bits_Data.obj, Bits_DataAnalysis.obj
#   in folder build/Cpp_Data.dir/debug
#---------------------------------------------------
set(SRC 
  src/Bits_Data.cpp
)
include_directories(src)
add_executable(CppData.exe ${SRC})
#---------------------------------------------------
