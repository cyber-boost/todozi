########## MACROS ###########################################################################
#############################################################################################

# Requires CMake > 3.15
if(${CMAKE_VERSION} VERSION_LESS "3.15")
    message(FATAL_ERROR "The 'CMakeDeps' generator only works with CMake >= 3.15")
endif()

if(todozi_FIND_QUIETLY)
    set(todozi_MESSAGE_MODE VERBOSE)
else()
    set(todozi_MESSAGE_MODE STATUS)
endif()

include(${CMAKE_CURRENT_LIST_DIR}/cmakedeps_macros.cmake)
include(${CMAKE_CURRENT_LIST_DIR}/todoziTargets.cmake)
include(CMakeFindDependencyMacro)

check_build_type_defined()

foreach(_DEPENDENCY ${todozi_FIND_DEPENDENCY_NAMES} )
    # Check that we have not already called a find_package with the transitive dependency
    if(NOT ${_DEPENDENCY}_FOUND)
        find_dependency(${_DEPENDENCY} REQUIRED ${${_DEPENDENCY}_FIND_MODE})
    endif()
endforeach()

set(todozi_VERSION_STRING "0.1.0")
set(todozi_INCLUDE_DIRS ${todozi_INCLUDE_DIRS_RELEASE} )
set(todozi_INCLUDE_DIR ${todozi_INCLUDE_DIRS_RELEASE} )
set(todozi_LIBRARIES ${todozi_LIBRARIES_RELEASE} )
set(todozi_DEFINITIONS ${todozi_DEFINITIONS_RELEASE} )


# Definition of extra CMake variables from cmake_extra_variables


# Only the last installed configuration BUILD_MODULES are included to avoid the collision
foreach(_BUILD_MODULE ${todozi_BUILD_MODULES_PATHS_RELEASE} )
    message(${todozi_MESSAGE_MODE} "Conan: Including build module from '${_BUILD_MODULE}'")
    include(${_BUILD_MODULE})
endforeach()


