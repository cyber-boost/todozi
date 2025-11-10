########### AGGREGATED COMPONENTS AND DEPENDENCIES FOR THE MULTI CONFIG #####################
#############################################################################################

list(APPEND todozi_COMPONENT_NAMES todozi::core todozi::agent todozi::api)
list(REMOVE_DUPLICATES todozi_COMPONENT_NAMES)
if(DEFINED todozi_FIND_DEPENDENCY_NAMES)
  list(APPEND todozi_FIND_DEPENDENCY_NAMES jansson CURL OpenSSL)
  list(REMOVE_DUPLICATES todozi_FIND_DEPENDENCY_NAMES)
else()
  set(todozi_FIND_DEPENDENCY_NAMES jansson CURL OpenSSL)
endif()
set(jansson_FIND_MODE "NO_MODULE")
set(CURL_FIND_MODE "NO_MODULE")
set(OpenSSL_FIND_MODE "NO_MODULE")

########### VARIABLES #######################################################################
#############################################################################################
set(todozi_PACKAGE_FOLDER_RELEASE "/root/.conan2/p/b/todoz93dee2d692d16/p")
set(todozi_BUILD_MODULES_PATHS_RELEASE )


set(todozi_INCLUDE_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/include")
set(todozi_RES_DIRS_RELEASE )
set(todozi_DEFINITIONS_RELEASE )
set(todozi_SHARED_LINK_FLAGS_RELEASE )
set(todozi_EXE_LINK_FLAGS_RELEASE )
set(todozi_OBJECTS_RELEASE )
set(todozi_COMPILE_DEFINITIONS_RELEASE )
set(todozi_COMPILE_OPTIONS_C_RELEASE )
set(todozi_COMPILE_OPTIONS_CXX_RELEASE )
set(todozi_LIB_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/lib")
set(todozi_BIN_DIRS_RELEASE )
set(todozi_LIBRARY_TYPE_RELEASE UNKNOWN)
set(todozi_IS_HOST_WINDOWS_RELEASE 0)
set(todozi_LIBS_RELEASE todozi_api todozi_agent todozi_core)
set(todozi_SYSTEM_LIBS_RELEASE )
set(todozi_FRAMEWORK_DIRS_RELEASE )
set(todozi_FRAMEWORKS_RELEASE )
set(todozi_BUILD_DIRS_RELEASE )
set(todozi_NO_SONAME_MODE_RELEASE FALSE)


# COMPOUND VARIABLES
set(todozi_COMPILE_OPTIONS_RELEASE
    "$<$<COMPILE_LANGUAGE:CXX>:${todozi_COMPILE_OPTIONS_CXX_RELEASE}>"
    "$<$<COMPILE_LANGUAGE:C>:${todozi_COMPILE_OPTIONS_C_RELEASE}>")
set(todozi_LINKER_FLAGS_RELEASE
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${todozi_SHARED_LINK_FLAGS_RELEASE}>"
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${todozi_SHARED_LINK_FLAGS_RELEASE}>"
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${todozi_EXE_LINK_FLAGS_RELEASE}>")


set(todozi_COMPONENTS_RELEASE todozi::core todozi::agent todozi::api)
########### COMPONENT todozi::api VARIABLES ############################################

set(todozi_todozi_api_INCLUDE_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/include")
set(todozi_todozi_api_LIB_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/lib")
set(todozi_todozi_api_BIN_DIRS_RELEASE )
set(todozi_todozi_api_LIBRARY_TYPE_RELEASE UNKNOWN)
set(todozi_todozi_api_IS_HOST_WINDOWS_RELEASE 0)
set(todozi_todozi_api_RES_DIRS_RELEASE )
set(todozi_todozi_api_DEFINITIONS_RELEASE )
set(todozi_todozi_api_OBJECTS_RELEASE )
set(todozi_todozi_api_COMPILE_DEFINITIONS_RELEASE )
set(todozi_todozi_api_COMPILE_OPTIONS_C_RELEASE "")
set(todozi_todozi_api_COMPILE_OPTIONS_CXX_RELEASE "")
set(todozi_todozi_api_LIBS_RELEASE todozi_api)
set(todozi_todozi_api_SYSTEM_LIBS_RELEASE )
set(todozi_todozi_api_FRAMEWORK_DIRS_RELEASE )
set(todozi_todozi_api_FRAMEWORKS_RELEASE )
set(todozi_todozi_api_DEPENDENCIES_RELEASE )
set(todozi_todozi_api_SHARED_LINK_FLAGS_RELEASE )
set(todozi_todozi_api_EXE_LINK_FLAGS_RELEASE )
set(todozi_todozi_api_NO_SONAME_MODE_RELEASE FALSE)

# COMPOUND VARIABLES
set(todozi_todozi_api_LINKER_FLAGS_RELEASE
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${todozi_todozi_api_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${todozi_todozi_api_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${todozi_todozi_api_EXE_LINK_FLAGS_RELEASE}>
)
set(todozi_todozi_api_COMPILE_OPTIONS_RELEASE
    "$<$<COMPILE_LANGUAGE:CXX>:${todozi_todozi_api_COMPILE_OPTIONS_CXX_RELEASE}>"
    "$<$<COMPILE_LANGUAGE:C>:${todozi_todozi_api_COMPILE_OPTIONS_C_RELEASE}>")
########### COMPONENT todozi::agent VARIABLES ############################################

set(todozi_todozi_agent_INCLUDE_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/include")
set(todozi_todozi_agent_LIB_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/lib")
set(todozi_todozi_agent_BIN_DIRS_RELEASE )
set(todozi_todozi_agent_LIBRARY_TYPE_RELEASE UNKNOWN)
set(todozi_todozi_agent_IS_HOST_WINDOWS_RELEASE 0)
set(todozi_todozi_agent_RES_DIRS_RELEASE )
set(todozi_todozi_agent_DEFINITIONS_RELEASE )
set(todozi_todozi_agent_OBJECTS_RELEASE )
set(todozi_todozi_agent_COMPILE_DEFINITIONS_RELEASE )
set(todozi_todozi_agent_COMPILE_OPTIONS_C_RELEASE "")
set(todozi_todozi_agent_COMPILE_OPTIONS_CXX_RELEASE "")
set(todozi_todozi_agent_LIBS_RELEASE todozi_agent)
set(todozi_todozi_agent_SYSTEM_LIBS_RELEASE )
set(todozi_todozi_agent_FRAMEWORK_DIRS_RELEASE )
set(todozi_todozi_agent_FRAMEWORKS_RELEASE )
set(todozi_todozi_agent_DEPENDENCIES_RELEASE )
set(todozi_todozi_agent_SHARED_LINK_FLAGS_RELEASE )
set(todozi_todozi_agent_EXE_LINK_FLAGS_RELEASE )
set(todozi_todozi_agent_NO_SONAME_MODE_RELEASE FALSE)

# COMPOUND VARIABLES
set(todozi_todozi_agent_LINKER_FLAGS_RELEASE
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${todozi_todozi_agent_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${todozi_todozi_agent_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${todozi_todozi_agent_EXE_LINK_FLAGS_RELEASE}>
)
set(todozi_todozi_agent_COMPILE_OPTIONS_RELEASE
    "$<$<COMPILE_LANGUAGE:CXX>:${todozi_todozi_agent_COMPILE_OPTIONS_CXX_RELEASE}>"
    "$<$<COMPILE_LANGUAGE:C>:${todozi_todozi_agent_COMPILE_OPTIONS_C_RELEASE}>")
########### COMPONENT todozi::core VARIABLES ############################################

set(todozi_todozi_core_INCLUDE_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/include")
set(todozi_todozi_core_LIB_DIRS_RELEASE "${todozi_PACKAGE_FOLDER_RELEASE}/lib")
set(todozi_todozi_core_BIN_DIRS_RELEASE )
set(todozi_todozi_core_LIBRARY_TYPE_RELEASE UNKNOWN)
set(todozi_todozi_core_IS_HOST_WINDOWS_RELEASE 0)
set(todozi_todozi_core_RES_DIRS_RELEASE )
set(todozi_todozi_core_DEFINITIONS_RELEASE )
set(todozi_todozi_core_OBJECTS_RELEASE )
set(todozi_todozi_core_COMPILE_DEFINITIONS_RELEASE )
set(todozi_todozi_core_COMPILE_OPTIONS_C_RELEASE "")
set(todozi_todozi_core_COMPILE_OPTIONS_CXX_RELEASE "")
set(todozi_todozi_core_LIBS_RELEASE todozi_core)
set(todozi_todozi_core_SYSTEM_LIBS_RELEASE )
set(todozi_todozi_core_FRAMEWORK_DIRS_RELEASE )
set(todozi_todozi_core_FRAMEWORKS_RELEASE )
set(todozi_todozi_core_DEPENDENCIES_RELEASE )
set(todozi_todozi_core_SHARED_LINK_FLAGS_RELEASE )
set(todozi_todozi_core_EXE_LINK_FLAGS_RELEASE )
set(todozi_todozi_core_NO_SONAME_MODE_RELEASE FALSE)

# COMPOUND VARIABLES
set(todozi_todozi_core_LINKER_FLAGS_RELEASE
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${todozi_todozi_core_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${todozi_todozi_core_SHARED_LINK_FLAGS_RELEASE}>
        $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${todozi_todozi_core_EXE_LINK_FLAGS_RELEASE}>
)
set(todozi_todozi_core_COMPILE_OPTIONS_RELEASE
    "$<$<COMPILE_LANGUAGE:CXX>:${todozi_todozi_core_COMPILE_OPTIONS_CXX_RELEASE}>"
    "$<$<COMPILE_LANGUAGE:C>:${todozi_todozi_core_COMPILE_OPTIONS_C_RELEASE}>")