use ndarray::Array2;
use std::f64;

// Profiler enum. We have two profilers: Cachegrind and Callgrind.
pub enum Profiler {
    // Cachgrind holds the parsed objects of
    // `valgrind --tool=cachegrind -cachegrind-out-file=cachegrind.out
    // && cg_annotate cachegrind.out`
    Cachegrind {
        ir: f64,
        i1mr: f64,
        ilmr: f64,
        dr: f64,
        d1mr: f64,
        dlmr: f64,
        dw: f64,
        d1mw: f64,
        dlmw: f64,
        data: Array2<f64>,
        functs: Vec<String>,
    },

    // Call holds the parsed objects of
    // `valgrind --tool=callgrind --callgrind-out-file=callgrind.out
    // && callgrind_annotate callgrind.out`
    Callgrind {
        total_instructions: f64,
        instructions: Vec<f64>,
        functs: Vec<String>,
    },
}

// Initialize the Profilers
impl Profiler {
    // Initialize Cachegrind

    pub fn new_cachegrind() -> Profiler {
        Profiler::Cachegrind {
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
            data: Array2::zeros((2, 2)),
            // profiled functions in binary
            functs: Vec::new(),
        }
    }
    // Initialize Callgrind
    pub fn new_callgrind() -> Profiler {
        Profiler::Callgrind {
            // total instruction calls
            total_instructions: f64::NAN,
            // instruction data
            instructions: Vec::new(),
            // profiled functions in binary
            functs: Vec::new(),
        }
    }
}
