### Compiling a C++ Program with a Statically- or Dynamically-Linked Rust Library Using CMake on Unix

This is an test minimal for adding Rust to a C++ project that is currently being compiled using CMake, through the ExternalProject module. Using this, it's possible to either statically or dynamically linked (using a `*.a` file) vs. dynamically-linked (using a `.so` file). Depending on which type is desired, change the `TARGET_LINK_LIBRARIES` command in the `CMakeLists.txt` file.

**NOTE: Static libraries can not specify their dependencies to the linker, and so need to be included explicity. This is also done using the `TARGET_LINK_LIBRARIES` command, including the dependencies *after* the static library file in the argument list**

The project can built by doing the following:
```
$ cmake .
$ make
```

The set-up is currently the following:
```    .
    ├── CMakeLists.txt
    ├── README.md
    ├── rust_hello
    │   ├── binding
    │   │   └── rust_hello.h
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    └── src
        └── main.cpp
```

The Rust library (`lib.rs`) is compiled using `cargo build` through the ExternalProject module in CMake, and has it's binding automatically generated using the `cbindgen` tool, with the resulting header file written to `binding/` artifacts set in the `target/` folder that gets created under `rust_hello`.

The `Cargo.toml` file for the `rust_hello` library specifies that both a static and dynamic library should be built, by specifying `crate-type = ["staticlib","cdylib"]` under `[lib]`.

The Rust library currently has two functions:
  - `hello_from_rust()` -> uses `println!()` to output "hello, world! -- from a Rust function!" to the command line
  - `return_a_four()` returns an `i32` type with a value of 4

Both of these should work as expected if using a dynamic library; however, if their dependencies are not explicitly declared to the linker *after* the linker sees the static libraries, the build process will throw and error during `make`. In this case, the use `println!()` seems to require `pthread` and `dl` to be included.


This project has been tested on a system with the following specs:
  - Ubuntu 18.04LTS
  - Intel i5 processor
  - cmake version 3.10.2
  - GNU make 4.1
  - g++/gcc version 7.3.0
  - cargo and rustc version 1.33.0 (stable)

The following resources also might be useful:

  - **cmake-cargo**: a tool for integrating cargo into cmake, which was used in the forked repo cited below. The `CMakeLists.txt` file is thoroughly commented, and a thread concerning its implementation is also cited below.
    - https://github.com/AndrewGaspar/cmake-cargo
    - https://github.com/AndrewGaspar/cmake_including_rust
    - https://users.rust-lang.org/t/issue-adding-a-rust-static-library-to-a-c-project-using-cmake/26478


  - Related Reddit thread about building in a Windows environment:
    - https://www.reddit.com/r/rust/comments/8vnf2h/statically_linking_a_rust_program_for_c/
    - https://github.com/TheYokai/rust-cpp-cmake

  - Rust Embedded's "A little Rust with your C":
    - https://rust-embedded.github.io/book/interoperability/rust-with-c.html

  - A project using CMake and Rust to run Rust on a BBC micro:bit (search for ExternalProject):
    - https://github.com/rossng/microbit-rust-hal
