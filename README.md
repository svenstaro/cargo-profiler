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
$ cargo-profiler callgrind --bin $PATH_TO_BINARY
$ cargo-profiler cachegrind --bin $PATH_TO_BINARY
```

You can limit the number of functions you'd like to look at:

```
$ cargo-profiler callgrind --bin $PATH_TO_BINARY -n 10

Profiling rsmat with callgrind...

Total Instructions...188599791

78346775 (41.5%) dgemm_kernel.rs:matrixmultiply::gemm::masked_kernel::h037533f74146b5d9
-----------------------------------------------------------------------
23528320 (12.5%) iter.rs:_$LT$std..ops..Range...std..iter..::next::h3637c345b34d1bb2
-----------------------------------------------------------------------
16824925 (8.9%) loopmacros.rs:matrixmultiply::gemm::masked_kernel::h037533f74146b5d9
-----------------------------------------------------------------------
10236864 (5.4%) mem.rs:core::mem::swap::h8d5cd2518659f0bb
-----------------------------------------------------------------------
7712846 (4.1%) memset.S:memset
-----------------------------------------------------------------------
6979680 (3.7%) ops.rs:_$LT$usize$u20$as$u20$ops..Add$GT$::add::hc57472a1060d1f70
-----------------------------------------------------------------------
6973791 (3.7%) ptr.rs:core::ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::offset::h686ec191b59fbed7
-----------------------------------------------------------------------
6049056 (3.2%) ops.rs:_$LT$$RF$$u27$b$u20$usize$u20$as$u20$ops..Add...::add::h09358de14d3353c1
-----------------------------------------------------------------------
3942400 (2.1%) wrapping.rs:_$LT$XorShiftRng$u20$as$u20$Rng$GT$::next_u32::h24beecf939a84404
-----------------------------------------------------------------------
3174400 (1.7%) mod.rs:core::num::_$LT$impl$u20$u32$GT$::overflowing_shr::haa481f134e0f4bd8
-----------------------------------------------------------------------

```

With cachegrind, you can also sort the data by a particular metric column:

```
$ cargo-profiler cachegrind --bin $PATH_TO_BINARY -n 10 --sort dw

Profiling rsmat with cachegrind...

Total Instructions...187979029	

Total I1 Read Misses...240	Total L1 Read Misses...222
Total D1 Reads...61037082	Total D1 Read Misses...48051
Total DL1 Read Misses...2	Total Writes...55133206
Total D1 Write Misses...10408	Total DL1 Write Misses...8449


 Ir  I1mr ILmr  Dr  D1mr DLmr  Dw  D1mw DLmw
0.42 0.28 0.30 0.38 0.93 1.00 0.38 0.00 0.00 dgemm_kernel.rs:matrixmultiply::gemm::masked_kernel...
-----------------------------------------------------------------------
0.12 0.03 0.02 0.11 0.00 0.00 0.15 0.00 0.00 iter.rs:_$LT$std..ops..Range...std..iter..Iterator$GT$...
-----------------------------------------------------------------------
0.05 0.01 0.01 0.08 0.00 0.00 0.08 0.00 0.00 mem.rs:core::mem::swap::h8d5cd2518659f0bb
-----------------------------------------------------------------------
0.02 0.02 0.01 0.02 0.00 0.00 0.04 0.00 0.00 wrapping.rs:_...::next_u32::h24beecf939a84404
-----------------------------------------------------------------------
0.04 0.01 0.01 0.04 0.00 0.00 0.03 0.00 0.00 ptr.rs:core::ptr::_...::offset::h686ec191b59fbed7
-----------------------------------------------------------------------
0.03 0.01 0.01 0.05 0.00 0.00 0.03 0.00 0.00 ops.rs:_...ops..Add...::add::h09358de14d3353c1
-----------------------------------------------------------------------
0.01 0.01 0.01 0.00 0.00 0.00 0.03 0.00 0.00 cmp.rs...::impls::_...$cmp..PartialOrd...usize$GT$::lt::...
-----------------------------------------------------------------------
0.01 0.03 0.03 0.01 0.00 0.00 0.03 0.00 0.00 lib.rs:_...::next_u32::h24beecf939a84404
-----------------------------------------------------------------------
0.01 0.00 0.00 0.00 0.00 0.00 0.03 0.00 0.00 ops.rs:_...ops..Add$GT$::add::hc57472a1060d1f70
-----------------------------------------------------------------------
0.09 0.06 0.06 0.13 0.00 0.00 0.02 0.00 0.00 loopmacros.rs:matrixmultiply::gemm::masked_kernel...
-----------------------------------------------------------------------

```



## TODO

* cmp subcommand - compare binary profiles
