cmake_minimum_required(VERSION 3.13)
set (CMAKE_CXX_STANDARD 11)

project(bigGraph)

add_executable(${PROJECT_NAME} main.cpp)

add_subdirectory(graph)
add_subdirectory(tests)

target_link_libraries(${PROJECT_NAME} graph tests)
