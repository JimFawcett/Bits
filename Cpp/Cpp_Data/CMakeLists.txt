#---------------------------------------------------
# HelloCMake - Demonstrate building C++ with CMake
#   - Non-hierarchal version
#---------------------------------------------------

#---------------------------------------------------
cmake_minimum_required(VERSION 3.25)
project(DataCMake)
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
#      -- Cpp_Data.dir directory
#         -- Debug directory 
#            -- Bits_Data.obj
#            -- Bits_DataAnalysis.obj 
#            -- ...
#---------------------------------------------------

set(CMake_CXX_STANDARD 20)

#---------------------------------------------------
# build Bits_Data.obj, Bits_DataAnalysis.obj
#   in folder build/Cpp_Data.dir/debug
#---------------------------------------------------
set(SRC 
  src/Bits_Data.cpp src/Bits_DataAnalysis.cpp
)
include_directories(src)
add_executable(Cpp_Data ${SRC})

#---------------------------------------------------
# To build library:
#---------------------------------------------------
# build Cpp_DataLib.lib in folder build/debug
#---------------------------------------------------
#add_library(
#  Cpp_DataLib STATIC libs/Cpp_Data_lib/Cpp_Data_lib.cpp
#)

#---------------------------------------------------
# link Bits_Data.obj and Cpp_Data_Lib.lib to 
# create Cpp_Data.exe in folder  build/debug
#---------------------------------------------------
#target_link_libraries(DataCMake Data_Lib)

#---------------------------------------------------
# Build process with CMake version 3.25
#---------------------------------------------------
#   1. add source files to project directories 
#   2. add CMakeLists.txt (this file)
#   3. mkdir build                 - this puts intermediate
#   4. cd build                      files in build directory
#   5. cmake ..                    - create cmake config files
#   6. cmake --build . [--config Debug | --config Release]
#   7. "./Debug/HelloCmake.exe"    - runs executable 
#   8. Notes:
#      - you can change any of the source files then: 
#          cmake --build . --config Debug
#          "./Debug/HelloCmake.exe
#      - delete contents of build directory to clean
#          cmake will regenerate
#   9. Note: 
#      - you can substitute Release for Debug 
#        in contents of 8.
#---------------------------------------------------
