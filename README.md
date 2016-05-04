# cargo-profiler
cargo subcommand to profile binaries

## To run

```
$ cargo build
$ ./target/debug/profiler --bin=$BINARY $PROFILER callgrind -n 10
$ ./target/debug/profiler --bin=$BINARY $PROFILER cachegrind -n 10

```



## TODO

* cmp subcommand - compare binary profiles
* sort output based on metric 
