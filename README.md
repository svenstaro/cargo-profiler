# cargo-profiler
cargo subcommand to profile binaries

## To run

```
$ cargo build
$ ./target/debug/profiler --bin=$BINARY callgrind -n 10
$ ./target/debug/profiler --bin=$BINARY cachegrind -n 10 --sort=ir

```



## TODO

* cmp subcommand - compare binary profiles
