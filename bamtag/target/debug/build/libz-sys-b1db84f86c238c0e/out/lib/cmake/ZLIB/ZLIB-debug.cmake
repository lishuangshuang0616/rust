#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "ZLIB::ZLIB" for configuration "Debug"
set_property(TARGET ZLIB::ZLIB APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(ZLIB::ZLIB PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/libz.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS ZLIB::ZLIB )
list(APPEND _IMPORT_CHECK_FILES_FOR_ZLIB::ZLIB "${_IMPORT_PREFIX}/lib/libz.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
