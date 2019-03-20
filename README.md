### Compiling a C++ Program with a Statically-Linked Rust Library Using CMake

#### Project Set-Up

This is an test case for adding Rust to a C++ project that is currently being
compiled using CMake, through the ExternalProject module. It's desired to have
this library be *statically* linked (using a `*.a` file) vs. dynamically-linked (using a `.so` file).

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

The Rust library (`lib.rs`) is compiled using `cargo build` through the ExternalProject module in CMake, and has it's binding automatically generated using the `cbindgen` tool, with the resulting artifacts set in the `target` folder that gets created under `rust_hello`.

The `Cargo.toml` file for that project specifies that both a static and dynamic library should be built, by specifying `crate-type = ["staticlib","cdylib"]` under `[lib]`. I have verified that both of these are created.

However, I begin running into an issue when I want to use the `println!()` macro. The Rust library currently has two functions:
  - `hello_from_rust()` -> uses `println!()` to output "hello, world! -- from a Rust function!" to the command line
  - `return_a_four()` returns an `i32` type with a value of 4

#### Current Issues

Everything works fine, as long as I am compiling with the dynamic `.so` library instead of the `.a` library. This gets specified near the end of the `CMakeLists.txt` file, (there's a TO_DO located above that section, which should be found fairly easily). When I compile using the dynamic library, everything works as expected such that I end up with a binary called `say_hello` in the main directory, that when run , and I get the following output:
```
username@user:~/rust-projects/cmake_including_rust$ ./say_hello
hello, world! -- from c++ main()
hello, world! -- from a Rust function!
my_int now equals: 4
                   ^ should be a '4'

```

However, if I start from the same clean version of the repository (clear all of the new cmake and cargo artifacts), and just alter the lines the `CMakeLists.txt` files s.t. they read

```
target_link_libraries(
  say_hello
  debug "${PROJECT_SOURCE_DIR}/rust_hello/target/debug/librust_hello.a"
  optimized "${PROJECT_SOURCE_DIR}/rust_hello/target/release/librust_hello.a"
)
```
and repeat the same build steps, I end up with the following during the `make` step:
```
username@user:~/rust-projects/cmake_including_rust$ make
Scanning dependencies of target rust_hello
[ 10%] Creating directories for 'rust_hello'
[ 20%] No download step for 'rust_hello'
[ 30%] No patch step for 'rust_hello'
[ 40%] No update step for 'rust_hello'
[ 50%] No configure step for 'rust_hello'
[ 60%] Performing build step for 'rust_hello'
-- rust_hello build command succeeded.  See also /home/username/rust-projects/cmake_including_rust/RUST/src/rust_hello-stamp/rust_hello-build-*.log
[ 70%] No install step for 'rust_hello'
[ 80%] Completed 'rust_hello'
[ 80%] Built target rust_hello
Scanning dependencies of target say_hello
[ 90%] Building CXX object CMakeFiles/say_hello.dir/src/main.cpp.o
[100%] Linking CXX executable say_hello
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::backtrace::printing::dladdr::resolve_symname':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/backtrace/printing/dladdr.rs:15: undefined reference to `dladdr'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::mutex::Mutex::init':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:45: undefined reference to `pthread_mutexattr_init'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:47: undefined reference to `pthread_mutexattr_settype'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:51: undefined reference to `pthread_mutexattr_destroy'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:45: undefined reference to `pthread_mutexattr_init'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:47: undefined reference to `pthread_mutexattr_settype'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:51: undefined reference to `pthread_mutexattr_destroy'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::mutex::ReentrantMutex::init':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:99: undefined reference to `pthread_mutexattr_init'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:101: undefined reference to `pthread_mutexattr_settype'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:106: undefined reference to `pthread_mutexattr_destroy'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:99: undefined reference to `pthread_mutexattr_init'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:101: undefined reference to `pthread_mutexattr_settype'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:106: undefined reference to `pthread_mutexattr_destroy'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::mutex::Mutex::init':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:45: undefined reference to `pthread_mutexattr_init'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:47: undefined reference to `pthread_mutexattr_settype'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/mutex.rs:51: undefined reference to `pthread_mutexattr_destroy'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::backtrace::printing::dladdr::resolve_symname':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/backtrace/printing/dladdr.rs:15: undefined reference to `dladdr'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::create':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::destroy':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::create':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::destroy':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::get':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:23: undefined reference to `pthread_getspecific'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::create':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::destroy':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::set':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:17: undefined reference to `pthread_setspecific'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::create':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::destroy':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::get':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:23: undefined reference to `pthread_getspecific'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:23: undefined reference to `pthread_getspecific'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::create':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:11: undefined reference to `pthread_key_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::destroy':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:28: undefined reference to `pthread_key_delete'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread_local::set':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread_local.rs:17: undefined reference to `pthread_setspecific'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::write':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:72: undefined reference to `pthread_rwlock_wrlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::raw_unlock':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::write':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:72: undefined reference to `pthread_rwlock_wrlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::raw_unlock':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::read':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:24: undefined reference to `pthread_rwlock_rdlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::raw_unlock':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/rwlock.rs:103: undefined reference to `pthread_rwlock_unlock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread::guard::get_stack_start':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:267: undefined reference to `pthread_getattr_np'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:271: undefined reference to `pthread_attr_getstack'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::weak::fetch':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/weak.rs:68: undefined reference to `dlsym'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::condvar::Condvar::init':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/condvar.rs:48: undefined reference to `pthread_condattr_setclock'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::process::process_inner::<impl std::sys::unix::process::process_common::Command>::do_exec':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/process/process_unix.rs:229: undefined reference to `pthread_sigmask'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread::pthread_attr_setstacksize':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:32: undefined reference to `pthread_attr_setstacksize'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread::Thread::new':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:64: undefined reference to `pthread_attr_setstacksize'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:69: undefined reference to `pthread_create'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread::Thread::join':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:168: undefined reference to `pthread_join'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `<std::sys::unix::thread::Thread as core::ops::drop::Drop>::drop':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:186: undefined reference to `pthread_detach'
rust_hello/target/release/librust_hello.a(std-e39317eb74365d3c.std.6yaz5a4r-cgu.0.rcgu.o): In function `std::sys::unix::thread::guard::current':
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:365: undefined reference to `pthread_getattr_np'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:368: undefined reference to `pthread_attr_getguardsize'
/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858//src/libstd/sys/unix/thread.rs:374: undefined reference to `pthread_attr_getstack'
collect2: error: ld returned 1 exit status
CMakeFiles/say_hello.dir/build.make:95: recipe for target 'say_hello' failed
make[2]: *** [say_hello] Error 1
CMakeFiles/Makefile2:67: recipe for target 'CMakeFiles/say_hello.dir/all' failed
make[1]: *** [CMakeFiles/say_hello.dir/all] Error 2
Makefile:83: recipe for target 'all' failed
make: *** [all] Error 2


```
What's more, if I comment out the `hello_from_rust()` function and comment it out in `main.cpp`, the `return_a_four()` function works fine! This makes me think it has something to the `println!()` function itself, and how that interacts with the environment itself. I don't even need to call the function or anything-- it's the build step which has the problem, so I don't think it's a runtime issue or anything along those lines.

I've done a bit of digging, but I can't seem to figure out why this is happening. It seems like there's a missing dependency in either `pthread` or with `dladdr`, although as far as I can tell, they're installed in my system and called out in the `CMakeLists.txt` file. I think something similar has been brought up in a Reddit thread (see below), although I believe the user was working on Windows (I'm on Linux), and I haven't been able to figure out the right combination of things to fix this problem (even though I'm mirroring his code pretty closely; thanks Eoin!)

Any help would be really appreciated!

Here are my current system specs:
  - Ubuntu 18.04LTS
  - Intel i5 processor
  - cmake version 3.10.2
  - GNU make 4.1
  - g++/gcc version 7.3.0
  - cargo and rustc version 1.33.0 (stable)

Reddit thread about this:
- https://www.reddit.com/r/rust/comments/8vnf2h/statically_linking_a_rust_program_for_c/
- https://github.com/TheYokai/rust-cpp-cmake

Rust Embedded's "A little Rust with your C":
- https://rust-embedded.github.io/book/interoperability/rust-with-c.html

A project using CMake and Rust to run Rust on a BBC micro:bit (search for ExternalProject):
- https://github.com/rossng/microbit-rust-hal
