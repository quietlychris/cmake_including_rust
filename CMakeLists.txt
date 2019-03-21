#--------------------------------------------------------
# The CMakeLists.txt for:
# Author(s):                              cmoran
#--------------------------------------------------------

cmake_minimum_required (VERSION 2.6)
project (say_hello)

include_directories ("${PROJECT_BINARY_DIR}")
include_directories ("${PROJECT_SOURCE_DIR}/rust_hello/binding")

SET(SRC
  src/main.cpp
)

ADD_EXECUTABLE(say_hello ${SRC})

include (ExternalProject)
set_directory_properties (PROPERTIES EP_PREFIX ${PROJECT_BINARY_DIR}/RUST)

ExternalProject_Add (
  rust_hello
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  BUILD_COMMAND "" cargo build COMMAND cargo build --release COMMAND cbindgen -o binding/rust_hello.h
  BINARY_DIR "${PROJECT_SOURCE_DIR}/rust_hello"
  INSTALL_COMMAND ""
  LOG_BUILD ON )

add_dependencies(say_hello rust_hello)

# Since static libraries can't communicate their own dependencies to the linker, we need to
# specify them explicitly, AFTER the static libraries (here, pthread and dl)
# We can ask Cargo for a list of the required dependencies by using the command
# `cargo rustc -- --print native-static-libs`, or by examining the error messages when
# using the `make` command.
TARGET_LINK_LIBRARIES(
  say_hello
  debug "${PROJECT_SOURCE_DIR}/rust_hello/target/debug/librust_hello.a"
  optimized "${PROJECT_SOURCE_DIR}/rust_hello/target/release/librust_hello.a"
  dl
  pthread
)

set_target_properties(say_hello PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
