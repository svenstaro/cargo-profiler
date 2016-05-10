#![feature(plugin)]
#![plugin(regex_macros)]

extern crate regex;
extern crate clap;
extern crate ndarray;

pub mod profiler;
pub mod parse;
pub mod display;
pub mod err;
pub mod argparse;
