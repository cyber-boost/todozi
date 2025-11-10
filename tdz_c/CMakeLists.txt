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

# Find dependencies - prefer Conan packages over system packages
if(CMAKE_TOOLCHAIN_FILE MATCHES "conan_toolchain")
    # When building with Conan, use Conan's packages
    find_package(jansson REQUIRED)
    find_package(CURL REQUIRED)
    find_package(OpenSSL REQUIRED)
    # TODO: Add UUID support back when linking issues are resolved
    # find_package(libuuid REQUIRED)
else()
    # Fallback to system packages when building without Conan
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(JANSSON REQUIRED jansson)
    pkg_check_modules(CURL REQUIRED libcurl)
    pkg_check_modules(OPENSSL REQUIRED openssl)
    pkg_check_modules(UUID REQUIRED uuid)
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
if(CMAKE_TOOLCHAIN_FILE MATCHES "conan_toolchain")
    # Link using Conan targets
    target_link_libraries(todozi PRIVATE
        jansson::jansson
        CURL::libcurl
        OpenSSL::SSL
        OpenSSL::Crypto
        # TODO: Add UUID support back when linking issues are resolved
        # libuuid::libuuid
    )
else()
    # Link using system packages
    target_link_libraries(todozi PRIVATE
        ${JANSSON_LIBRARIES}
        ${CURL_LIBRARIES}
        ${OPENSSL_LIBRARIES}
        ${UUID_LIBRARIES}
    )

    # Include directories for system dependencies
    target_include_directories(todozi PRIVATE
        ${JANSSON_INCLUDE_DIRS}
        ${CURL_INCLUDE_DIRS}
        ${OPENSSL_INCLUDE_DIRS}
        ${UUID_INCLUDE_DIRS}
    )
endif()

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
    # Find all example files recursively
    file(GLOB_RECURSE EXAMPLE_SOURCES "examples/*.c")

    foreach(EXAMPLE_SOURCE ${EXAMPLE_SOURCES})
        get_filename_component(EXAMPLE_NAME ${EXAMPLE_SOURCE} NAME_WE)
        get_filename_component(EXAMPLE_DIR ${EXAMPLE_SOURCE} DIRECTORY)
        get_filename_component(EXAMPLE_PARENT_DIR ${EXAMPLE_DIR} NAME)

        # Create unique target name with directory prefix
        set(EXAMPLE_TARGET "${EXAMPLE_PARENT_DIR}_${EXAMPLE_NAME}")

        add_executable(${EXAMPLE_TARGET} ${EXAMPLE_SOURCE})
        target_link_libraries(${EXAMPLE_TARGET} PRIVATE todozi)

        # Set output directory for examples
        set_target_properties(${EXAMPLE_TARGET} PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/examples/${EXAMPLE_PARENT_DIR}"
        )
    endforeach()
endif()
