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


set(CPACK_BUILD_SOURCE_DIRS "C:/Users/lishuangshuang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libz-ng-sys-1.1.15/src/zlib-ng;D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build")
set(CPACK_CMAKE_GENERATOR "Visual Studio 17 2022")
set(CPACK_COMPONENT_UNSPECIFIED_HIDDEN "TRUE")
set(CPACK_COMPONENT_UNSPECIFIED_REQUIRED "TRUE")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_FILE "D:/LenovoSoftstore/Install/CMake/share/cmake-3.31/Templates/CPack.GenericDescription.txt")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_SUMMARY "zlib built using CMake")
set(CPACK_GENERATOR "TGZ")
set(CPACK_INNOSETUP_ARCHITECTURE "x64")
set(CPACK_INSTALL_CMAKE_PROJECTS "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build;zlib;ALL;/")
set(CPACK_INSTALL_PREFIX "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out")
set(CPACK_MODULE_PATH "")
set(CPACK_NSIS_DISPLAY_NAME "zlib-ng 2.1.6")
set(CPACK_NSIS_INSTALLER_ICON_CODE "")
set(CPACK_NSIS_INSTALLER_MUI_ICON_CODE "")
set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES64")
set(CPACK_NSIS_PACKAGE_NAME "zlib-ng 2.1.6")
set(CPACK_NSIS_UNINSTALL_NAME "Uninstall")
set(CPACK_OUTPUT_CONFIG_FILE "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build/CPackConfig.cmake")
set(CPACK_PACKAGE_DEFAULT_LOCATION "/")
set(CPACK_PACKAGE_DESCRIPTION_FILE "D:/LenovoSoftstore/Install/CMake/share/cmake-3.31/Templates/CPack.GenericDescription.txt")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "zlib built using CMake")
set(CPACK_PACKAGE_DIRECTORY "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build/package")
set(CPACK_PACKAGE_FILE_NAME "zlib-ng-2.1.6-win64")
set(CPACK_PACKAGE_INSTALL_DIRECTORY "zlib-ng 2.1.6")
set(CPACK_PACKAGE_INSTALL_REGISTRY_KEY "zlib-ng 2.1.6")
set(CPACK_PACKAGE_NAME "zlib-ng")
set(CPACK_PACKAGE_RELOCATABLE "true")
set(CPACK_PACKAGE_VENDOR "Humanity")
set(CPACK_PACKAGE_VERSION "2.1.6")
set(CPACK_PACKAGE_VERSION_MAJOR "1")
set(CPACK_PACKAGE_VERSION_MINOR "3")
set(CPACK_PACKAGE_VERSION_PATCH "0")
set(CPACK_RESOURCE_FILE_LICENSE "D:/LenovoSoftstore/Install/CMake/share/cmake-3.31/Templates/CPack.GenericLicense.txt")
set(CPACK_RESOURCE_FILE_README "D:/LenovoSoftstore/Install/CMake/share/cmake-3.31/Templates/CPack.GenericDescription.txt")
set(CPACK_RESOURCE_FILE_WELCOME "D:/LenovoSoftstore/Install/CMake/share/cmake-3.31/Templates/CPack.GenericWelcome.txt")
set(CPACK_SET_DESTDIR "OFF")
set(CPACK_SOURCE_GENERATOR "TGZ")
set(CPACK_SOURCE_IGNORE_FILES ".git/;_CPack_Packages/;D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build/")
set(CPACK_SOURCE_OUTPUT_CONFIG_FILE "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build/CPackSourceConfig.cmake")
set(CPACK_SYSTEM_NAME "win64")
set(CPACK_THREADS "1")
set(CPACK_TOPLEVEL_TAG "win64")
set(CPACK_WIX_SIZEOF_VOID_P "8")

if(NOT CPACK_PROPERTIES_FILE)
  set(CPACK_PROPERTIES_FILE "D:/Files/share/lishuangshuang/pipeline/learning/rust/hello/target/debug/build/libz-ng-sys-57566e67ad75161a/out/build/CPackProperties.cmake")
endif()

if(EXISTS ${CPACK_PROPERTIES_FILE})
  include(${CPACK_PROPERTIES_FILE})
endif()
