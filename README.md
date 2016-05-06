# cargo-profiler
cargo subcommand to profile binaries

## To install

First install valgrind:

```
$ sudo apt-get install valgrind
```

Then, build the binary.
```
$ cargo build --release
```

Now, either copy the built binary to the same directory as cargo, or call the binary directly. 

```
$ sudo cp ./target/release/cargo-profiler $(dirname $(which cargo))/
$ cargo-profiler $ARGS
```

or 
```
$ ./target/release/cargo-profiler $ARGS
```

## To run

Cargo profiler currently supports callgrind and cachegrind. 

```
$ cargo-profiler callgrind --bin=$PATH_TO_BINARY
$ cargo-profiler cachegrind --bin=$PATH_TO_BINARY
```

You can limit the number of functions you'd like to look at:

```
$ cargo-profiler callgrind --bin=$PATH_TO_BINARY -n 10
$ cargo-profiler cachegrind --bin=$PATH_TO_BINARY -n 10
```

With cachegrind, you can also sort the data by a particular metric column:

```
$ cargo-profiler cachegrind --bin=$PATH_TO_BINARY --sort ir
$ cargo-profiler cachegrind --bin=$PATH_TO_BINARY -n 10 --sort dw
```



## TODO

* cmp subcommand - compare binary profiles
