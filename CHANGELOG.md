* 0.1.6 - Arguments to binary can be supplied.
* 0.1.5 - Bug fix
* 0.1.4 - Detects invocation outside of rust project. Propagates valgrind memory error up to cargo profiler. Moves process exit/print error function to main.rs so we can create unit tests for underlying functions. This is achieved by a `try_or_exit` macro in main.rs. Generally better error handling with result/option combinators (e.g. and_then, ok_or, etc.). Unit tests initialized in each submodule.
* 0.1.3 - cargo better integrated. No longer have to specify binary if in rust project w/ cargo.toml. better error messages and exits (e.g. upon compilation errors).
