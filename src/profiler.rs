extern crate ndarray;
use self::ndarray::{OwnedArray, Ix};
use std::f64;

// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;

// Profiler enum. We have two profilers: CacheGrind and CallGrind.
pub enum Profiler {
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
        functs: Vec<String>,
    },

    // Call holds the parsed objects of
    // `valgrind --tool=callgrind --callgrind-out-file=callgrind.out
    // && callgrind_annotate callgrind.out`
    CallGrind {
        total_instructions: f64,
        instructions: Vec<f64>,
        functs: Vec<String>,
    },
}


// Initialize the Profilers
impl Profiler {
    // Initialize CacheGrind

    pub fn new_cachegrind() -> Profiler {
        Profiler::CacheGrind {
            // total instructions
            ir: f64::NAN,
            // total instruction-cache read misses
            i1mr: f64::NAN,
            // total LL-cache read misses
            ilmr: f64::NAN,
            // total reads
            dr: f64::NAN,
            // total data-cache read misses
            d1mr: f64::NAN,
            // total LL-cache read misses
            dlmr: f64::NAN,
            // total data-cache writes
            dw: f64::NAN,
            // total data-cache write-misses
            d1mw: f64::NAN,
            // total LL cache write misses
            dlmw: f64::NAN,
            // profiler data
            data: OwnedArray::zeros((2, 2)),
            // profiled functions in binary
            functs: Vec::new(),
        }
    }
    // Initialize CallGrind
    pub fn new_callgrind() -> Profiler {
        Profiler::CallGrind {
            // total instruction calls
            total_instructions: f64::NAN,
            // instruction data
            instructions: Vec::new(),
            // profiled functions in binary
            functs: Vec::new(),
        }
    }
}
