#[allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate clap;
extern crate ndarray;
extern crate rustc_serialize;

pub mod profiler;
pub mod display;
pub mod err;
pub mod argparse;
pub mod cargo;

pub mod parse {
    pub mod callgrind;
    pub mod cachegrind;
}
