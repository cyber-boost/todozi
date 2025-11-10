cmake_minimum_required(VERSION 3.16)
project(todozi VERSION 0.1.0 LANGUAGES C)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Compiler warnings and optimizations
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Include directories
include_directories(include)

# Find dependencies
find_package(PkgConfig REQUIRED)
pkg_check_modules(JANSSON REQUIRED jansson)
pkg_check_modules(CURL REQUIRED libcurl)
pkg_check_modules(OPENSSL REQUIRED openssl)
pkg_check_modules(UUID REQUIRED uuid)

# Conan will provide these automatically
if(CONAN_EXPORTED)
    include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
    conan_basic_setup(TARGETS)
endif()

# Collect source files (excluding main.c and files with their own main)
set(TODOZI_SOURCES
    src/agent.c
    src/api.c
    src/base.c
    src/chunking.c
    src/cli.c
    src/emb.c
    src/error.c
    src/extract.c
    src/idea.c
    src/lib.c
    src/memory.c
    src/migration.c
    src/models.c
    src/reminder.c
    src/search.c
    src/server.c
    src/storage.c
    src/summary.c
    src/tags.c
    src/tdz.c
    src/tdz_tls.c
    src/tests.c
    src/todozi.c
    src/todozi_exe.c
    src/todozi_tool.c
    src/tui.c
    src/types.c
)

# Create static library
add_library(todozi STATIC ${TODOZI_SOURCES})

# Link dependencies
target_link_libraries(todozi PRIVATE
    ${JANSSON_LIBRARIES}
    ${CURL_LIBRARIES}
    ${OPENSSL_LIBRARIES}
    ${UUID_LIBRARIES}
)

# Include directories for dependencies
target_include_directories(todozi PRIVATE
    ${JANSSON_INCLUDE_DIRS}
    ${CURL_INCLUDE_DIRS}
    ${OPENSSL_INCLUDE_DIRS}
    ${UUID_INCLUDE_DIRS}
)

# Create executable
add_executable(todozi_exe src/main.c)
target_link_libraries(todozi_exe PRIVATE todozi)
set_target_properties(todozi_exe PROPERTIES OUTPUT_NAME "todozi")

# Install targets
install(TARGETS todozi
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

install(TARGETS todozi_exe
    RUNTIME DESTINATION bin
)

install(DIRECTORY include/
    DESTINATION include
    FILES_MATCHING PATTERN "*.h"
)

# Examples (optional)
option(BUILD_EXAMPLES "Build examples" OFF)
if(BUILD_EXAMPLES)
    file(GLOB EXAMPLE_SOURCES "examples/*.c")
    foreach(EXAMPLE_SOURCE ${EXAMPLE_SOURCES})
        get_filename_component(EXAMPLE_NAME ${EXAMPLE_SOURCE} NAME_WE)
        add_executable(${EXAMPLE_NAME} ${EXAMPLE_SOURCE})
        target_link_libraries(${EXAMPLE_NAME} PRIVATE todozi)
    endforeach()
endif()
