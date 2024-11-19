# This file will be configured to contain variables for CPack. These variables
# should be set in the CMake list file of the project before CPack module is
# included. The list of available CPACK_xxx variables and their associated
# documentation may be obtained using
#  cpack --help-variable-list
#
# Some variables are common to all generators (e.g. CPACK_PACKAGE_NAME)
# and some are specific to a generator
# (e.g. CPACK_NSIS_EXTRA_INSTALL_COMMANDS). The generator specific variables
# usually begin with CPACK_<GENNAME>_xxxx.


set(CPACK_BUILD_SOURCE_DIRS "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libz-sys-1.1.16/src/zlib-ng;/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build")
set(CPACK_CMAKE_GENERATOR "Unix Makefiles")
set(CPACK_COMPONENT_UNSPECIFIED_HIDDEN "TRUE")
set(CPACK_COMPONENT_UNSPECIFIED_REQUIRED "TRUE")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_FILE "/usr/share/cmake-3.22/Templates/CPack.GenericDescription.txt")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_SUMMARY "zlib built using CMake")
set(CPACK_GENERATOR "TGZ")
set(CPACK_INSTALL_CMAKE_PROJECTS "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build;zlib;ALL;/")
set(CPACK_INSTALL_PREFIX "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out")
set(CPACK_MODULE_PATH "")
set(CPACK_NSIS_DISPLAY_NAME "zlib 1.3.0.zlib-ng")
set(CPACK_NSIS_INSTALLER_ICON_CODE "")
set(CPACK_NSIS_INSTALLER_MUI_ICON_CODE "")
set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES")
set(CPACK_NSIS_PACKAGE_NAME "zlib 1.3.0.zlib-ng")
set(CPACK_NSIS_UNINSTALL_NAME "Uninstall")
set(CPACK_OUTPUT_CONFIG_FILE "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build/CPackConfig.cmake")
set(CPACK_PACKAGE_DEFAULT_LOCATION "/")
set(CPACK_PACKAGE_DESCRIPTION_FILE "/usr/share/cmake-3.22/Templates/CPack.GenericDescription.txt")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "zlib built using CMake")
set(CPACK_PACKAGE_DIRECTORY "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build/package")
set(CPACK_PACKAGE_FILE_NAME "zlib-1.3.0.zlib-ng-Linux")
set(CPACK_PACKAGE_INSTALL_DIRECTORY "zlib 1.3.0.zlib-ng")
set(CPACK_PACKAGE_INSTALL_REGISTRY_KEY "zlib 1.3.0.zlib-ng")
set(CPACK_PACKAGE_NAME "zlib")
set(CPACK_PACKAGE_RELOCATABLE "true")
set(CPACK_PACKAGE_VENDOR "Humanity")
set(CPACK_PACKAGE_VERSION "1.3.0.zlib-ng")
set(CPACK_PACKAGE_VERSION_MAJOR "1")
set(CPACK_PACKAGE_VERSION_MINOR "3")
set(CPACK_PACKAGE_VERSION_PATCH "0")
set(CPACK_RESOURCE_FILE_LICENSE "/usr/share/cmake-3.22/Templates/CPack.GenericLicense.txt")
set(CPACK_RESOURCE_FILE_README "/usr/share/cmake-3.22/Templates/CPack.GenericDescription.txt")
set(CPACK_RESOURCE_FILE_WELCOME "/usr/share/cmake-3.22/Templates/CPack.GenericWelcome.txt")
set(CPACK_SET_DESTDIR "OFF")
set(CPACK_SOURCE_GENERATOR "TGZ")
set(CPACK_SOURCE_IGNORE_FILES ".git/;_CPack_Packages/;/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build/")
set(CPACK_SOURCE_OUTPUT_CONFIG_FILE "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build/CPackSourceConfig.cmake")
set(CPACK_SYSTEM_NAME "Linux")
set(CPACK_THREADS "1")
set(CPACK_TOPLEVEL_TAG "Linux")
set(CPACK_WIX_SIZEOF_VOID_P "8")

if(NOT CPACK_PROPERTIES_FILE)
  set(CPACK_PROPERTIES_FILE "/mnt/hgfs/lishuangshuang/pipeline/learning/rust/bamtag/target/debug/build/libz-sys-b1db84f86c238c0e/out/build/CPackProperties.cmake")
endif()

if(EXISTS ${CPACK_PROPERTIES_FILE})
  include(${CPACK_PROPERTIES_FILE})
endif()