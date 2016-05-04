# cargo-profiler
cargo subcommand to profile binaries

## To run

```
$ cargo build
$ ./target/debug/profiler --bin=$BINARY $PROFILER callgrind -n 10
$ ./target/debug/profiler --bin=$BINARY $PROFILER cachegrind -n 15

```

## TODO

* cmp subcommand - compare binary profiles
* save to file - save profile to file
* sort output by metric 
