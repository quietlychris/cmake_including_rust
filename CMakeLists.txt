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

TARGET_LINK_LIBRARIES(say_hello
   dl
   pthread
)

include (ExternalProject)
set_directory_properties (PROPERTIES EP_PREFIX ${PROJECT_BINARY_DIR}/RUST)

ExternalProject_Add (
  rust_hello
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  BUILD_COMMAND cargo build COMMAND cargo build --release COMMAND cbindgen -o binding/rust_hello.h
  BINARY_DIR "${PROJECT_SOURCE_DIR}/rust_hello"
  INSTALL_COMMAND ""
  LOG_BUILD ON )

add_dependencies(say_hello rust_hello)

# TO_DO: Figure out why `.a` doesn't work, but `.so` does
# If we use the dynamic '.so' library rather than static '.a', the 'hello_from_rust()' will work
target_link_libraries(
  say_hello
  debug "${PROJECT_SOURCE_DIR}/rust_hello/target/debug/librust_hello.so"
  optimized "${PROJECT_SOURCE_DIR}/rust_hello/target/release/librust_hello.so"
)

set_target_properties(say_hello PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
