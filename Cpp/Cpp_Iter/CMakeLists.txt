#---------------------------------------------------
# HelloCMake - Demonstrate building C++ with CMake
#   - Non-hierarchal version
#---------------------------------------------------

#---------------------------------------------------
cmake_minimum_required(VERSION 3.30.0-rc3)
project(IterCMake)
#---------------------------------------------------
#   Cpp_Objects dir
#   -- CMakeLists.txt (this file)
#   -- src dir
#      -- Cpp_Objects.cpp
#   -- build directory
#      -- Debug directory 
#         -- Cpp_Objects.exe 
#         -- ...
#      -- Cpp_Objects.dir directory
#         -- Debug directory 
#            -- Cpp_Hello.obj 

#---------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

#---------------------------------------------------
# build HelloCMake.obj in folder build/Cpp_Hello.dir/debug
#---------------------------------------------------
add_executable(Cpp_Iter src/Bits_Iter.cpp)
if (MSVC)
  # suppress warning about need for C++20 for concepts
  add_compile_options(/wd4038)
endif()
#---------------------------------------------------
# build HelloCMakeLib.lib in folder build/debug
#---------------------------------------------------
#add_library(HelloCMakeLib STATIC libs/hello_lib/hello_lib.cpp)

#---------------------------------------------------
# link HelloCMake.obj and HelloCMakeLib.lib to 
# create HelloCMake.exe in folder  build/debug
#---------------------------------------------------
#target_link_libraries(HelloCMake HelloCMakeLib)

#---------------------------------------------------
# Build process with CMake version 3.18.2
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
