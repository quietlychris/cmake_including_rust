rm -r RUST
rm -r CMakeFiles
rm cmake_install.cmake
rm Makefile
rm CMakeCache.txt
rm say_hello

cd rust_hello; cargo clean; rm Cargo.lock
