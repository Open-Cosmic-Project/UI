cmake_minimum_required(VERSION 3.20)
project(CreationDE LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Core Gui Quick Qml WaylandClient)

set(PROJECT_SOURCES
    src/main.cpp
    qml/qml.qrc
)

qt_add_executable(creationde ${PROJECT_SOURCES})

target_link_libraries(creationde PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Quick
    Qt6::Qml
    Qt6::WaylandClient
)
