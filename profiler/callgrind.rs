extern crate ndarray;

use std::process::Command;
use std::fmt;
use self::ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};
use std::f64;

pub struct CallGrind<'a> {
    total_instructions: f64,
    instructions: Vec<f64>,
    functs: Option<Vec<&'a str>>,
}

impl<'a> CallGrind<'a> {
    pub fn new_callgrind() -> CallGrind<'a> {
        CallGrind {
            // total instruction calls
            total_instructions: f64::NAN,
            // instruction data
            instructions: vec![0.],
            // profiled functions in binary
            functs: None,
        }
    }
}

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait CallGrindParser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str,num: usize) -> CallGrind;
}

impl<'a> CallGrindParser for CallGrind<'a> {
    fn cli(&self, binary: &str) -> String {
        Command::new("valgrind")
            .arg("--tool=callgrind")
            .arg("--callgrind-out-file=callgrind.out")
            .arg(binary)
            .output()
            .unwrap_or_else(|e| panic!("failed {}", e));
        let cachegrind_output = Command::new("callgrind_annotate")
                                    .arg("callgrind.out")
                                    .arg(binary)
                                    .output()
                                    .unwrap_or_else(|e| panic!("failed {}", e));
        String::from_utf8(cachegrind_output.stdout).expect("cli error")
    }

    fn parse<'b>(&'b self, output: &'b str, num: usize) -> CallGrind {
        {

            // split output line-by-line
            let mut out_split = output.split("\n").collect::<Vec<_>>();

            // regex identifies lines that start with digits and have characters that commonly
            // show up in file paths
            let re = regex!(r"\d+\s*[a-zA-Z]*$*_*:*/+\.*");
            out_split.retain(|x| re.is_match(x));


            let mut funcs: Vec<&'b str> = Vec::new();
            let mut data: Vec<f64> = Vec::new();
            // loop through each line and get numbers + func
            for sample in out_split.iter() {

                // trim the sample, split by whitespace to separate out each data point
                // (numbers + func)
                let elems = sample.trim().split("  ").collect::<Vec<_>>();

                // for each number, remove any commas and parse into f64. the last element in
                // data_elems is the function file path.
                if let Ok(s) = elems[0]
                                   .trim()
                                   .replace(",", "")
                                   .parse::<f64>() {
                    data.push(s);
                }

                // the function has some trailing whitespace and trash. remove that, and
                // get the function, push to functs vector.
                let path = elems[1].split(" ").collect::<Vec<_>>();
                let sp = path[0].split("/").collect::<Vec<_>>();
                funcs.push(sp[sp.len() - 1])

            }

            // get the total instructions by summing the data vector.
            let total_instructions = data.iter().fold(0.0, |a, b| a + b);

            // parse the limit argument n, and take the first n values of data/funcs vectors
            // accordingly.

            if num < data.len() {
                data = data.iter().take(num).cloned().collect();
                funcs = funcs.iter().take(num).cloned().collect();
            }
            // put all data in cachegrind struct!
            CallGrind {
                total_instructions:total_instructions,
                instructions: data,
                functs: Some(funcs),
            }
        }
}
}
