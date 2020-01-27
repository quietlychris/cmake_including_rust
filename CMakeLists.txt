#--------------------------------------------------------
# The CMakeLists.txt for:
# Author(s):                              cmoran
#--------------------------------------------------------
set(PROJ_NAME say_hello)
set(RUST_DIR rust_hello)

cmake_minimum_required (VERSION 2.6)
project (${PROJ_NAME})

include_directories ("${PROJECT_BINARY_DIR}")
include_directories ("${PROJECT_SOURCE_DIR}/${RUST_DIR}/binding")

SET(SRC src/main.cpp)

ADD_EXECUTABLE(say_hello ${SRC})

include (ExternalProject)
set_directory_properties (PROPERTIES EP_PREFIX ${PROJECT_BINARY_DIR}/RUST)

ExternalProject_Add (
  ${RUST_DIR}
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  BUILD_COMMAND "" cargo build COMMAND cargo build --release COMMAND cbindgen -o binding/${RUST_DIR}.h
  BINARY_DIR "${PROJECT_SOURCE_DIR}/${RUST_DIR}"
  INSTALL_COMMAND ""
  LOG_BUILD ON )

add_dependencies(${PROJ_NAME} ${RUST_DIR})

# Since static libraries can't communicate their own dependencies to the linker, we need to
# specify them explicitly, AFTER the static libraries (here, pthread and dl)
# We can ask Cargo for a list of the required dependencies by using the command
# `cargo rustc -- --print native-static-libs`, or by examining the error messages when
# using the `make` command.
TARGET_LINK_LIBRARIES(
  say_hello
  debug "${PROJECT_SOURCE_DIR}/${RUST_DIR}/target/debug/lib${RUST_DIR}.a"
  optimized "${PROJECT_SOURCE_DIR}/${RUST_DIR}/target/release/lib${RUST_DIR}.a"
  dl
  pthread
)

set_target_properties(${RUST_DIR} PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
