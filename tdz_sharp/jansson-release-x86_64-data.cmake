########### AGGREGATED COMPONENTS AND DEPENDENCIES FOR THE MULTI CONFIG #####################
#############################################################################################

set(jansson_COMPONENT_NAMES "")
if(DEFINED jansson_FIND_DEPENDENCY_NAMES)
  list(APPEND jansson_FIND_DEPENDENCY_NAMES )
  list(REMOVE_DUPLICATES jansson_FIND_DEPENDENCY_NAMES)
else()
  set(jansson_FIND_DEPENDENCY_NAMES )
endif()

########### VARIABLES #######################################################################
#############################################################################################
set(jansson_PACKAGE_FOLDER_RELEASE "/root/.conan2/p/b/janss759119dfcca63/p")
set(jansson_BUILD_MODULES_PATHS_RELEASE )


set(jansson_INCLUDE_DIRS_RELEASE "${jansson_PACKAGE_FOLDER_RELEASE}/include")
set(jansson_RES_DIRS_RELEASE )
set(jansson_DEFINITIONS_RELEASE )
set(jansson_SHARED_LINK_FLAGS_RELEASE )
set(jansson_EXE_LINK_FLAGS_RELEASE )
set(jansson_OBJECTS_RELEASE )
set(jansson_COMPILE_DEFINITIONS_RELEASE )
set(jansson_COMPILE_OPTIONS_C_RELEASE )
set(jansson_COMPILE_OPTIONS_CXX_RELEASE )
set(jansson_LIB_DIRS_RELEASE "${jansson_PACKAGE_FOLDER_RELEASE}/lib")
set(jansson_BIN_DIRS_RELEASE )
set(jansson_LIBRARY_TYPE_RELEASE STATIC)
set(jansson_IS_HOST_WINDOWS_RELEASE 0)
set(jansson_LIBS_RELEASE jansson)
set(jansson_SYSTEM_LIBS_RELEASE )
set(jansson_FRAMEWORK_DIRS_RELEASE )
set(jansson_FRAMEWORKS_RELEASE )
set(jansson_BUILD_DIRS_RELEASE )
set(jansson_NO_SONAME_MODE_RELEASE FALSE)


# COMPOUND VARIABLES
set(jansson_COMPILE_OPTIONS_RELEASE
    "$<$<COMPILE_LANGUAGE:CXX>:${jansson_COMPILE_OPTIONS_CXX_RELEASE}>"
    "$<$<COMPILE_LANGUAGE:C>:${jansson_COMPILE_OPTIONS_C_RELEASE}>")
set(jansson_LINKER_FLAGS_RELEASE
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${jansson_SHARED_LINK_FLAGS_RELEASE}>"
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${jansson_SHARED_LINK_FLAGS_RELEASE}>"
    "$<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${jansson_EXE_LINK_FLAGS_RELEASE}>")


set(jansson_COMPONENTS_RELEASE )