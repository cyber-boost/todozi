# Avoid multiple calls to find_package to append duplicated properties to the targets
include_guard()########### VARIABLES #######################################################################
#############################################################################################
set(jansson_FRAMEWORKS_FOUND_RELEASE "") # Will be filled later
conan_find_apple_frameworks(jansson_FRAMEWORKS_FOUND_RELEASE "${jansson_FRAMEWORKS_RELEASE}" "${jansson_FRAMEWORK_DIRS_RELEASE}")

set(jansson_LIBRARIES_TARGETS "") # Will be filled later


######## Create an interface target to contain all the dependencies (frameworks, system and conan deps)
if(NOT TARGET jansson_DEPS_TARGET)
    add_library(jansson_DEPS_TARGET INTERFACE IMPORTED)
endif()

set_property(TARGET jansson_DEPS_TARGET
             APPEND PROPERTY INTERFACE_LINK_LIBRARIES
             $<$<CONFIG:Release>:${jansson_FRAMEWORKS_FOUND_RELEASE}>
             $<$<CONFIG:Release>:${jansson_SYSTEM_LIBS_RELEASE}>
             $<$<CONFIG:Release>:>)

####### Find the libraries declared in cpp_info.libs, create an IMPORTED target for each one and link the
####### jansson_DEPS_TARGET to all of them
conan_package_library_targets("${jansson_LIBS_RELEASE}"    # libraries
                              "${jansson_LIB_DIRS_RELEASE}" # package_libdir
                              "${jansson_BIN_DIRS_RELEASE}" # package_bindir
                              "${jansson_LIBRARY_TYPE_RELEASE}"
                              "${jansson_IS_HOST_WINDOWS_RELEASE}"
                              jansson_DEPS_TARGET
                              jansson_LIBRARIES_TARGETS  # out_libraries_targets
                              "_RELEASE"
                              "jansson"    # package_name
                              "${jansson_NO_SONAME_MODE_RELEASE}")  # soname

# FIXME: What is the result of this for multi-config? All configs adding themselves to path?
set(CMAKE_MODULE_PATH ${jansson_BUILD_DIRS_RELEASE} ${CMAKE_MODULE_PATH})

########## GLOBAL TARGET PROPERTIES Release ########################################
    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                 $<$<CONFIG:Release>:${jansson_OBJECTS_RELEASE}>
                 $<$<CONFIG:Release>:${jansson_LIBRARIES_TARGETS}>
                 )

    if("${jansson_LIBS_RELEASE}" STREQUAL "")
        # If the package is not declaring any "cpp_info.libs" the package deps, system libs,
        # frameworks etc are not linked to the imported targets and we need to do it to the
        # global target
        set_property(TARGET jansson::jansson
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     jansson_DEPS_TARGET)
    endif()

    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_LINK_OPTIONS
                 $<$<CONFIG:Release>:${jansson_LINKER_FLAGS_RELEASE}>)
    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_INCLUDE_DIRECTORIES
                 $<$<CONFIG:Release>:${jansson_INCLUDE_DIRS_RELEASE}>)
    # Necessary to find LINK shared libraries in Linux
    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_LINK_DIRECTORIES
                 $<$<CONFIG:Release>:${jansson_LIB_DIRS_RELEASE}>)
    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_COMPILE_DEFINITIONS
                 $<$<CONFIG:Release>:${jansson_COMPILE_DEFINITIONS_RELEASE}>)
    set_property(TARGET jansson::jansson
                 APPEND PROPERTY INTERFACE_COMPILE_OPTIONS
                 $<$<CONFIG:Release>:${jansson_COMPILE_OPTIONS_RELEASE}>)

########## For the modules (FindXXX)
set(jansson_LIBRARIES_RELEASE jansson::jansson)
