# cargo-profiler
cargo subcommand to profile binaries

## To run

```
$ cargo build
$ ./target/debug/profiler --bin=$BINARY $PROFILER
```
Currently support perf and cachegrind overall statistics, as well as callgrind function records.

If using callgrind function records, you can limit output with

```
$ ./target/debug/profiler --bin=$BINARY $PROFILER callgrind -n 10
```

## TODO

* cmp subcommand - compare binary profiles
* save to file - save profile to file

* Zero-in on expensive functions.
  * Print the line number of the functions, and/or whether they are internal or external to the library
  * Decompose expensive functions even further based on docs?
