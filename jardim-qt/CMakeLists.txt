cmake_minimum_required(VERSION 3.21)
project(jardim VERSION 0.3.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Mesmas otimizações da versão Rust: O3, LTO, código ajustado ao Cortex-A53.
set(CMAKE_CXX_FLAGS_RELEASE "-O3 -DNDEBUG")
if(ANDROID)
  add_compile_options(-mcpu=cortex-a53)
endif()
include(CheckIPOSupported)
check_ipo_supported(RESULT ipo_ok OUTPUT ipo_msg)

find_package(Qt6 6.8 REQUIRED COMPONENTS Quick Svg)
qt_standard_project_setup(REQUIRES 6.8)

qt_add_executable(jardim
  src/main.cpp src/app.cpp src/app.h src/curriculum.cpp src/curriculum.h src/art.cpp src/art.h)

set_source_files_properties(qml/Theme.qml PROPERTIES QT_QML_SINGLETON_TYPE TRUE)
qt_add_qml_module(jardim
  URI Jardim
  VERSION 1.0
  QML_FILES qml/Main.qml qml/Theme.qml qml/Art.qml qml/Btn.qml qml/Logo.qml qml/Header.qml
            qml/Home.qml qml/GameCard.qml qml/Game.qml qml/Visual.qml qml/SceneView.qml qml/Finish.qml)

target_link_libraries(jardim PRIVATE Qt6::Quick Qt6::Svg)
if(ipo_ok)
  set_property(TARGET jardim PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
endif()

set_target_properties(jardim PROPERTIES
  QT_ANDROID_PACKAGE_NAME "org.jardim.descobertas.qt"
  QT_ANDROID_APP_NAME "Jardim (Qt)"
  QT_ANDROID_APP_ICON "@drawable/ic_launcher"
  QT_ANDROID_VERSION_CODE 3
  QT_ANDROID_VERSION_NAME "0.3.0-qt"
  QT_ANDROID_MIN_SDK_VERSION 28
  QT_ANDROID_TARGET_SDK_VERSION 35
  QT_ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android")
