# Avoid multiple calls to find_package to append duplicated properties to the targets
include_guard()########### VARIABLES #######################################################################
#############################################################################################
set(todozi_FRAMEWORKS_FOUND_RELEASE "") # Will be filled later
conan_find_apple_frameworks(todozi_FRAMEWORKS_FOUND_RELEASE "${todozi_FRAMEWORKS_RELEASE}" "${todozi_FRAMEWORK_DIRS_RELEASE}")

set(todozi_LIBRARIES_TARGETS "") # Will be filled later


######## Create an interface target to contain all the dependencies (frameworks, system and conan deps)
if(NOT TARGET todozi_DEPS_TARGET)
    add_library(todozi_DEPS_TARGET INTERFACE IMPORTED)
endif()

set_property(TARGET todozi_DEPS_TARGET
             APPEND PROPERTY INTERFACE_LINK_LIBRARIES
             $<$<CONFIG:Release>:${todozi_FRAMEWORKS_FOUND_RELEASE}>
             $<$<CONFIG:Release>:${todozi_SYSTEM_LIBS_RELEASE}>
             $<$<CONFIG:Release>:jansson::jansson;CURL::libcurl;openssl::openssl>)

####### Find the libraries declared in cpp_info.libs, create an IMPORTED target for each one and link the
####### todozi_DEPS_TARGET to all of them
conan_package_library_targets("${todozi_LIBS_RELEASE}"    # libraries
                              "${todozi_LIB_DIRS_RELEASE}" # package_libdir
                              "${todozi_BIN_DIRS_RELEASE}" # package_bindir
                              "${todozi_LIBRARY_TYPE_RELEASE}"
                              "${todozi_IS_HOST_WINDOWS_RELEASE}"
                              todozi_DEPS_TARGET
                              todozi_LIBRARIES_TARGETS  # out_libraries_targets
                              "_RELEASE"
                              "todozi"    # package_name
                              "${todozi_NO_SONAME_MODE_RELEASE}")  # soname

# FIXME: What is the result of this for multi-config? All configs adding themselves to path?
set(CMAKE_MODULE_PATH ${todozi_BUILD_DIRS_RELEASE} ${CMAKE_MODULE_PATH})

########## COMPONENTS TARGET PROPERTIES Release ########################################

    ########## COMPONENT todozi::api #############

        set(todozi_todozi_api_FRAMEWORKS_FOUND_RELEASE "")
        conan_find_apple_frameworks(todozi_todozi_api_FRAMEWORKS_FOUND_RELEASE "${todozi_todozi_api_FRAMEWORKS_RELEASE}" "${todozi_todozi_api_FRAMEWORK_DIRS_RELEASE}")

        set(todozi_todozi_api_LIBRARIES_TARGETS "")

        ######## Create an interface target to contain all the dependencies (frameworks, system and conan deps)
        if(NOT TARGET todozi_todozi_api_DEPS_TARGET)
            add_library(todozi_todozi_api_DEPS_TARGET INTERFACE IMPORTED)
        endif()

        set_property(TARGET todozi_todozi_api_DEPS_TARGET
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_api_FRAMEWORKS_FOUND_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_api_SYSTEM_LIBS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_api_DEPENDENCIES_RELEASE}>
                     )

        ####### Find the libraries declared in cpp_info.component["xxx"].libs,
        ####### create an IMPORTED target for each one and link the 'todozi_todozi_api_DEPS_TARGET' to all of them
        conan_package_library_targets("${todozi_todozi_api_LIBS_RELEASE}"
                              "${todozi_todozi_api_LIB_DIRS_RELEASE}"
                              "${todozi_todozi_api_BIN_DIRS_RELEASE}" # package_bindir
                              "${todozi_todozi_api_LIBRARY_TYPE_RELEASE}"
                              "${todozi_todozi_api_IS_HOST_WINDOWS_RELEASE}"
                              todozi_todozi_api_DEPS_TARGET
                              todozi_todozi_api_LIBRARIES_TARGETS
                              "_RELEASE"
                              "todozi_todozi_api"
                              "${todozi_todozi_api_NO_SONAME_MODE_RELEASE}")


        ########## TARGET PROPERTIES #####################################
        set_property(TARGET todozi::api
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_api_OBJECTS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_api_LIBRARIES_TARGETS}>
                     )

        if("${todozi_todozi_api_LIBS_RELEASE}" STREQUAL "")
            # If the component is not declaring any "cpp_info.components['foo'].libs" the system, frameworks etc are not
            # linked to the imported targets and we need to do it to the global target
            set_property(TARGET todozi::api
                         APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                         todozi_todozi_api_DEPS_TARGET)
        endif()

        set_property(TARGET todozi::api APPEND PROPERTY INTERFACE_LINK_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_api_LINKER_FLAGS_RELEASE}>)
        set_property(TARGET todozi::api APPEND PROPERTY INTERFACE_INCLUDE_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_api_INCLUDE_DIRS_RELEASE}>)
        set_property(TARGET todozi::api APPEND PROPERTY INTERFACE_LINK_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_api_LIB_DIRS_RELEASE}>)
        set_property(TARGET todozi::api APPEND PROPERTY INTERFACE_COMPILE_DEFINITIONS
                     $<$<CONFIG:Release>:${todozi_todozi_api_COMPILE_DEFINITIONS_RELEASE}>)
        set_property(TARGET todozi::api APPEND PROPERTY INTERFACE_COMPILE_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_api_COMPILE_OPTIONS_RELEASE}>)


    ########## COMPONENT todozi::agent #############

        set(todozi_todozi_agent_FRAMEWORKS_FOUND_RELEASE "")
        conan_find_apple_frameworks(todozi_todozi_agent_FRAMEWORKS_FOUND_RELEASE "${todozi_todozi_agent_FRAMEWORKS_RELEASE}" "${todozi_todozi_agent_FRAMEWORK_DIRS_RELEASE}")

        set(todozi_todozi_agent_LIBRARIES_TARGETS "")

        ######## Create an interface target to contain all the dependencies (frameworks, system and conan deps)
        if(NOT TARGET todozi_todozi_agent_DEPS_TARGET)
            add_library(todozi_todozi_agent_DEPS_TARGET INTERFACE IMPORTED)
        endif()

        set_property(TARGET todozi_todozi_agent_DEPS_TARGET
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_agent_FRAMEWORKS_FOUND_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_agent_SYSTEM_LIBS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_agent_DEPENDENCIES_RELEASE}>
                     )

        ####### Find the libraries declared in cpp_info.component["xxx"].libs,
        ####### create an IMPORTED target for each one and link the 'todozi_todozi_agent_DEPS_TARGET' to all of them
        conan_package_library_targets("${todozi_todozi_agent_LIBS_RELEASE}"
                              "${todozi_todozi_agent_LIB_DIRS_RELEASE}"
                              "${todozi_todozi_agent_BIN_DIRS_RELEASE}" # package_bindir
                              "${todozi_todozi_agent_LIBRARY_TYPE_RELEASE}"
                              "${todozi_todozi_agent_IS_HOST_WINDOWS_RELEASE}"
                              todozi_todozi_agent_DEPS_TARGET
                              todozi_todozi_agent_LIBRARIES_TARGETS
                              "_RELEASE"
                              "todozi_todozi_agent"
                              "${todozi_todozi_agent_NO_SONAME_MODE_RELEASE}")


        ########## TARGET PROPERTIES #####################################
        set_property(TARGET todozi::agent
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_agent_OBJECTS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_agent_LIBRARIES_TARGETS}>
                     )

        if("${todozi_todozi_agent_LIBS_RELEASE}" STREQUAL "")
            # If the component is not declaring any "cpp_info.components['foo'].libs" the system, frameworks etc are not
            # linked to the imported targets and we need to do it to the global target
            set_property(TARGET todozi::agent
                         APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                         todozi_todozi_agent_DEPS_TARGET)
        endif()

        set_property(TARGET todozi::agent APPEND PROPERTY INTERFACE_LINK_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_agent_LINKER_FLAGS_RELEASE}>)
        set_property(TARGET todozi::agent APPEND PROPERTY INTERFACE_INCLUDE_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_agent_INCLUDE_DIRS_RELEASE}>)
        set_property(TARGET todozi::agent APPEND PROPERTY INTERFACE_LINK_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_agent_LIB_DIRS_RELEASE}>)
        set_property(TARGET todozi::agent APPEND PROPERTY INTERFACE_COMPILE_DEFINITIONS
                     $<$<CONFIG:Release>:${todozi_todozi_agent_COMPILE_DEFINITIONS_RELEASE}>)
        set_property(TARGET todozi::agent APPEND PROPERTY INTERFACE_COMPILE_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_agent_COMPILE_OPTIONS_RELEASE}>)


    ########## COMPONENT todozi::core #############

        set(todozi_todozi_core_FRAMEWORKS_FOUND_RELEASE "")
        conan_find_apple_frameworks(todozi_todozi_core_FRAMEWORKS_FOUND_RELEASE "${todozi_todozi_core_FRAMEWORKS_RELEASE}" "${todozi_todozi_core_FRAMEWORK_DIRS_RELEASE}")

        set(todozi_todozi_core_LIBRARIES_TARGETS "")

        ######## Create an interface target to contain all the dependencies (frameworks, system and conan deps)
        if(NOT TARGET todozi_todozi_core_DEPS_TARGET)
            add_library(todozi_todozi_core_DEPS_TARGET INTERFACE IMPORTED)
        endif()

        set_property(TARGET todozi_todozi_core_DEPS_TARGET
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_core_FRAMEWORKS_FOUND_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_core_SYSTEM_LIBS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_core_DEPENDENCIES_RELEASE}>
                     )

        ####### Find the libraries declared in cpp_info.component["xxx"].libs,
        ####### create an IMPORTED target for each one and link the 'todozi_todozi_core_DEPS_TARGET' to all of them
        conan_package_library_targets("${todozi_todozi_core_LIBS_RELEASE}"
                              "${todozi_todozi_core_LIB_DIRS_RELEASE}"
                              "${todozi_todozi_core_BIN_DIRS_RELEASE}" # package_bindir
                              "${todozi_todozi_core_LIBRARY_TYPE_RELEASE}"
                              "${todozi_todozi_core_IS_HOST_WINDOWS_RELEASE}"
                              todozi_todozi_core_DEPS_TARGET
                              todozi_todozi_core_LIBRARIES_TARGETS
                              "_RELEASE"
                              "todozi_todozi_core"
                              "${todozi_todozi_core_NO_SONAME_MODE_RELEASE}")


        ########## TARGET PROPERTIES #####################################
        set_property(TARGET todozi::core
                     APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                     $<$<CONFIG:Release>:${todozi_todozi_core_OBJECTS_RELEASE}>
                     $<$<CONFIG:Release>:${todozi_todozi_core_LIBRARIES_TARGETS}>
                     )

        if("${todozi_todozi_core_LIBS_RELEASE}" STREQUAL "")
            # If the component is not declaring any "cpp_info.components['foo'].libs" the system, frameworks etc are not
            # linked to the imported targets and we need to do it to the global target
            set_property(TARGET todozi::core
                         APPEND PROPERTY INTERFACE_LINK_LIBRARIES
                         todozi_todozi_core_DEPS_TARGET)
        endif()

        set_property(TARGET todozi::core APPEND PROPERTY INTERFACE_LINK_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_core_LINKER_FLAGS_RELEASE}>)
        set_property(TARGET todozi::core APPEND PROPERTY INTERFACE_INCLUDE_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_core_INCLUDE_DIRS_RELEASE}>)
        set_property(TARGET todozi::core APPEND PROPERTY INTERFACE_LINK_DIRECTORIES
                     $<$<CONFIG:Release>:${todozi_todozi_core_LIB_DIRS_RELEASE}>)
        set_property(TARGET todozi::core APPEND PROPERTY INTERFACE_COMPILE_DEFINITIONS
                     $<$<CONFIG:Release>:${todozi_todozi_core_COMPILE_DEFINITIONS_RELEASE}>)
        set_property(TARGET todozi::core APPEND PROPERTY INTERFACE_COMPILE_OPTIONS
                     $<$<CONFIG:Release>:${todozi_todozi_core_COMPILE_OPTIONS_RELEASE}>)


    ########## AGGREGATED GLOBAL TARGET WITH THE COMPONENTS #####################
    set_property(TARGET todozi::todozi APPEND PROPERTY INTERFACE_LINK_LIBRARIES todozi::api)
    set_property(TARGET todozi::todozi APPEND PROPERTY INTERFACE_LINK_LIBRARIES todozi::agent)
    set_property(TARGET todozi::todozi APPEND PROPERTY INTERFACE_LINK_LIBRARIES todozi::core)

########## For the modules (FindXXX)
set(todozi_LIBRARIES_RELEASE todozi::todozi)
