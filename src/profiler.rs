extern crate ndarray;
use self::ndarray::{OwnedArray, Ix};
use std::f64;

// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;

// Profiler enum. We have two profilers: CacheGrind and CallGrind.
pub enum Profiler<'a> {
    // CachGrind holds the parsed objects of
    // `valgrind --tool=cachegrind -cachegrind-out-file=cachegrind.out
    // && cg_annotate cachegrind.out`
    CacheGrind {
        ir: f64,
        i1mr: f64,
        ilmr: f64,
        dr: f64,
        d1mr: f64,
        dlmr: f64,
        dw: f64,
        d1mw: f64,
        dlmw: f64,
        data: Mat<f64>,
        functs: Option<Vec<&'a str>>,
    },

    // Call holds the parsed objects of
    // `valgrind --tool=callgrind --callgrind-out-file=callgrind.out
    // && callgrind_annotate callgrind.out`
    CallGrind {
        total_instructions: f64,
        instructions: Vec<f64>,
        functs: Option<Vec<&'a str>>,
    },
}


// Initialize the Profilers
impl<'a> Profiler<'a> {
    // Initialize CacheGrind
    pub fn new_cachegrind() -> Profiler<'a> {
        Profiler::CacheGrind {
            // total instruction references
            ir: f64::NAN,
            // total i1-cache read misses
            i1mr: f64::NAN,
            // total iL-cache read misses
            ilmr: f64::NAN,
            // total reads
            dr: f64::NAN,
            // total d1-cache read misses
            d1mr: f64::NAN,
            // total dL-cache read misses
            dlmr: f64::NAN,
            // total d-cache writes
            dw: f64::NAN,
            // total d1-cache write-misses
            d1mw: f64::NAN,
            // total dL cache write misses
            dlmw: f64::NAN,
            // profiler data
            data: OwnedArray::zeros((2, 2)),
            // profiled functions in binary
            functs: None,
        }
    }
    // Initialize CallGrind
    pub fn new_callgrind() -> Profiler<'a> {
        Profiler::CallGrind {
            // total instruction calls
            total_instructions: f64::NAN,
            // instruction data
            instructions: vec![0.],
            // profiled functions in binary
            functs: None,
        }
    }
}
