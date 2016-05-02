# cargo-profiler
cargo subcommand to profile binaries

## To run

```
$ cargo build
$ ./target/debug/profiler --bin=$BINARY --profiler=$PROFILER
```
Currently support perf and cachegrind overall statistics, as well as callgrind function records.

## TODO

* Pretty print callgrind overall statistics.
* Zero-in on expensive functions.
  * Print how much of the total instructions they make up.
  * Print the line number of the functions, and/or whether they are internal or external to the library
  * Decompose expensive functions even further based on docs? 
