#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;
extern crate itertools;
extern crate ndarray;

use std::process::Command;
use std::fmt;
use ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};

// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;

// utility function for sorting a matrix. used to sort cachegrind data by particular metric.
pub fn sort_matrix(mat : Mat<f64>, sort_col: ArrayView<f64,Ix>) -> (Mat<f64>, Vec<usize>){
    let mut enum_col = sort_col.iter().enumerate().collect::<Vec<(usize, &f64)>>();
    enum_col.sort_by(|a, &b| a.1.partial_cmp(b.1).unwrap());
    let indices = enum_col.iter().map(|x| x.0).collect::<Vec<usize>>();
    (mat.select(Axis(0), indices.as_slice()), indices)
}

// Profiler enum. We have two profilers: CacheGrind and CallGrind.
pub enum Profiler<'a> {
    // CachGrind holds the parsed objects of
    // `valgrind --tool=cachegrind -cachegrind-out-file=cachegrind.out && cg_annotate
    // cachegrind.out`
    CacheGrind {
        ir: Option<f64>,
        i1mr: Option<f64>,
        ilmr: Option<f64>,
        dr: Option<f64>,
        d1mr: Option<f64>,
        dlmr: Option<f64>,
        dw: Option<f64>,
        d1mw: Option<f64>,
        dlmw: Option<f64>,
        data: Option<Mat<f64>>,
        functs: Option<Vec<&'a str>>,
    },

    // Call holds the parsed objects of
    // `valgrind --tool=callgrind --callgrind-out-file=callgrind.out && callgrind_annotate
    // callgrind.out`
    CallGrind {
        total_instructions: Option<f64>,
        instructions: Option<Vec<f64>>,
        functs: Option<Vec<&'a str>>,
    },
}


// Initialize the Profilers
impl<'a> Profiler<'a> {
    // Initialize CacheGrind
    pub fn new_cachegrind() -> Profiler<'a> {
        Profiler::CacheGrind {
            // total instruction references
            ir: None,
            // total i1-cache read misses
            i1mr: None,
            // total iL-cache read misses
            ilmr: None,
            // total reads
            dr: None,
            // total d1-cache read misses
            d1mr: None,
            // total dL-cache read misses
            dlmr: None,
            // total d-cache writes
            dw: None,
            // total d1-cache write-misses
            d1mw: None,
            // total dL cache write misses
            dlmw: None,
            // profiler data
            data: None,
            // profiled functions in binary
            functs: None,
        }
    }
    // Initialize CallGrind
    pub fn new_callgrind() -> Profiler<'a> {
        Profiler::CallGrind {
            // total instruction calls
            total_instructions: None,
            // instruction data
            instructions: None,
            // profiled functions in binary
            functs: None,
        }
    }
}

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait Parser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str,n: &str, s: &str) -> Profiler;
}



impl<'a> Parser for Profiler<'a> {
    // Get profiler output from stdout.
    fn cli(&self, binary: &str) -> String {
        match *self {

            // get cachegrind cli output from stdout
            Profiler::CacheGrind { .. } => {
                Command::new("valgrind")
                    .arg("--tool=cachegrind")
                    .arg("--cachegrind-out-file=cachegrind.out")
                    .arg(binary)
                    .output()
                    .unwrap_or_else(|e| panic!("failed {}", e));
                let cachegrind_output = Command::new("cg_annotate")
                                            .arg("cachegrind.out")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| panic!("failed {}", e));
                String::from_utf8(cachegrind_output.stdout).expect("cli error")
            }

            // get callgrind cli output from stdout
            Profiler::CallGrind { .. } => {
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
        }

    }

    // Get parse the profiler output into respective structs.
    fn parse<'b>(&'b self, output: &'b str, n: &str, s: &str) -> Profiler {
        match *self {

            Profiler::CacheGrind { .. } => {
                // split output line-by-line
                let mut out_split: Vec<&'b str> = output.split("\n").collect();

                // regex identifies lines that start with digits and have characters that commonly
                // show up in file paths
                let re = regex!(r"\d+\s*[a-zA-Z]*$*_*:*/+\.*");
                out_split.retain(|x| re.is_match(x));


                let mut funcs: Vec<&str> = Vec::new();
                let mut data: Vec<Mat<f64>> = Vec::new();
                // loop through each line and get numbers + func
                for sample in out_split.iter() {

                    // trim the sample, split by whitespace to separate out each data point
                    // (numbers + func)
                    let mut data_elems = sample.trim()
                                               .split(" ")
                                               .collect::<Vec<_>>();

                    // remove any empty strings
                    data_elems.retain(|x| x.to_string() != "");

                    // for each number, remove any commas and parse into f64. the last element in
                    // data_elems is the function file path.
                    let numbers = data_elems[0..data_elems.len() - 1]
                                      .iter()
                                      .map(|x| x.trim().replace(",", "").parse::<f64>().unwrap())
                                      .collect::<Vec<f64>>();

                    // reshape the vector of parsed numbers into a 1 x 9 matrix, and push the
                    // matrix to our vector of 1 x 9 matrices.
                    if let Ok(data_col) = OwnedArray::from_shape_vec((numbers.len(), 1), numbers) {
                        data.push(data_col);
                    }
                    // the last element in data_elems is the function file path.
                    // get the file in the file-path (which includes the function) and push that to
                    // the funcs vector.
                    let sp = data_elems[data_elems.len() - 1].split("/").collect::<Vec<_>>();
                    funcs.push(sp[sp.len() - 1]);
                }

                // stack all the 1 x 9 matrices in data to a 9 x n  matrix.
                let mat = stack(Axis(1),
                                &data.iter().map(|x| x.view()).collect::<Vec<_>>().as_slice())
                              .ok()
                              .unwrap();
                // transpose the matrix so we have a n x 9 matrix displayed.
                let mat = mat.t();

                // match the sort argument to a column of the matrix that we will sort on.
                // default sorting -> first column (total instructions).
                let sort_col = match s {
                    "ir" => mat.column(0),
                    "i1mr" => mat.column(1),
                    "ilmr" => mat.column(2),
                    "dr" => mat.column(3),
                    "d1mr" => mat.column(4),
                    "dlmr" => mat.column(5),
                    "dw" => mat.column(6),
                    "d1mw" => mat.column(7),
                    "dlmw" => mat.column(8),
                    "none" => mat.column(0),
                    _ => panic!("sort argument is not valid"),
                };

                // sort the matrix of data and functions by a particular column.
                // to sort matrix, we keep track of sorted indices, and select the matrix wrt
                // these sorted indices. to sort functions, we index the funcs vector with the
                // sorted indices.
                let (mut sorted_funcs, mut mat) = match s {
                    "none" => (funcs, mat.to_owned()),
                    _ => {
                        let (mat, indices) = sort_matrix(mat.to_owned(),sort_col);
                        (indices.iter().map(|&x| funcs[x]).collect::<Vec<&'b str>>(), mat)

                    }
                };

                // also, when we sort, we want to make sure that we reverse the order of the
                // matrix/funcs so that the most expensive functions show up at the top.
                match s {
                    "none" => {}
                    _ => {
                        let mut reverse_indices = (0..mat.rows()).collect::<Vec<usize>>();
                        reverse_indices.reverse();
                        mat = mat.select(Axis(0), reverse_indices.as_slice());
                        &sorted_funcs.reverse();
                    }
                }

                // sum the columns of the data matrix to get total metrics.
                let ir = mat.column(0).scalar_sum();
                let i1mr = mat.column(1).scalar_sum();
                let ilmr = mat.column(2).scalar_sum();
                let dr = mat.column(3).scalar_sum();
                let d1mr = mat.column(4).scalar_sum();
                let dlmr = mat.column(5).scalar_sum();
                let dw = mat.column(6).scalar_sum();
                let d1mw = mat.column(7).scalar_sum();
                let dlmw = mat.column(8).scalar_sum();


                // parse the limit argument n, and take the first n values of data matrix/funcs
                // vector accordingly.
                if let Ok(s) = n.parse::<usize>() {
                    if s < mat.rows() {
                        let ls = (0..s).collect::<Vec<_>>();

                        mat = mat.select(Axis(0), ls.as_slice());

                        sorted_funcs = sorted_funcs.iter()
                                                   .take(s)
                                                   .map(|x| x.to_owned())
                                                   .collect();
                    }

                }

                // put all data in cachegrind struct!
                Profiler::CacheGrind {
                    ir: Some(ir),
                    i1mr: Some(i1mr),
                    ilmr: Some(ilmr),
                    dr: Some(dr),
                    d1mr: Some(d1mr),
                    dlmr: Some(dlmr),
                    dw: Some(dw),
                    d1mw: Some(d1mw),
                    dlmw: Some(dlmw),
                    data: Some(mat),
                    functs: Some(sorted_funcs),
                }
            }

            Profiler::CallGrind { .. } => {

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
                if let Ok(s) = n.parse::<usize>() {
                    if s < data.len() {
                        data = data.iter().take(s).map(|x| x.to_owned()).collect();
                        funcs = funcs.iter().take(s).map(|x| x.to_owned()).collect();
                    }

                }

                // put all data in cachegrind struct!
                Profiler::CallGrind {
                    total_instructions: Some(total_instructions),
                    instructions: Some(data),
                    functs: Some(funcs),
                }
            }

        }
    }
}


// Pretty-print the profiler outputs into user-friendly formats.
impl<'a> fmt::Display for Profiler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Profiler::CacheGrind { ref ir,
                                   ref i1mr,
                                   ref ilmr,
                                   ref dr,
                                   ref d1mr,
                                   ref dlmr,
                                   ref dw,
                                   ref d1mw,
                                   ref dlmw,
                                   ref data,
                                   ref functs } => {
                write!(f,
                       "\n\x1b[32mTotal Instructions\x1b[0m...{:#}\t\x1b[0m\n\n\
                       \x1b[32mTotal I1 Read Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal L1 Read Misses\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal D1 Reads\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal D1 Read Misses\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal DL1 Read Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal Writes\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal D1 Write Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal DL1 Write Misses\x1b[0m...{}\x1b[0m\n\n\n",
                       ir.unwrap_or(std::f64::NAN),
                       i1mr.unwrap_or(std::f64::NAN),
                       ilmr.unwrap_or(std::f64::NAN),
                       dr.unwrap_or(std::f64::NAN),
                       d1mr.unwrap_or(std::f64::NAN),
                       dlmr.unwrap_or(std::f64::NAN),
                       dw.unwrap_or(std::f64::NAN),
                       d1mw.unwrap_or(std::f64::NAN),
                       dlmw.unwrap_or(std::f64::NAN),
                   );
                write!(f,
                       " \x1b[1;36mIr  \x1b[1;36mI1mr \x1b[1;36mILmr  \x1b[1;36mDr  \
                        \x1b[1;36mD1mr \x1b[1;36mDLmr  \x1b[1;36mDw  \x1b[1;36mD1mw \
                        \x1b[1;36mDLmw\n");

                if let &Some(ref func) = functs {
                    if let &Some(ref dat) = data {

                        for (ref x, &y) in dat.axis_iter(Axis(0)).zip(func.iter()) {
                            write!(f,
                                   "\x1b[0m{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} \
                                    {}\n",
                                   x[0] / ir.unwrap() as f64,
                                   x[1] / i1mr.unwrap(),
                                   x[2] / ilmr.unwrap() as f64,
                                   x[3] / dr.unwrap() as f64,
                                   x[4] / d1mr.unwrap() as f64,
                                   x[5] / dlmr.unwrap() as f64,
                                   x[6] / dw.unwrap() as f64,
                                   x[7] / d1mw.unwrap() as f64,
                                   x[8] / dlmw.unwrap() as f64,
                                   y);
                            println!("-----------------------------------------------------------------------");


                        }
                    }
                }
                Ok(())
            }

            Profiler::CallGrind { ref total_instructions, ref instructions, ref functs } => {

                write!(f,
                       "\n\x1b[32mTotal Instructions\x1b[0m...{}\n\n\x1b[0m",
                       total_instructions.unwrap_or(std::f64::NAN));

                if let &Some(ref func) = functs {
                    if let &Some(ref ins) = instructions {
                        for (&x, &y) in ins.iter().zip(func.iter()) {
                            {

                                let perc = x / total_instructions.unwrap_or(std::f64::NAN) as f64 *
                                           100.;
                                match perc {
                                    t if t >= 50.0 => {
                                        write!(f,
                                               "{} (\x1b[31m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               t,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }
                                    t if (t >= 30.0) & (t < 50.0) => {
                                        write!(f,
                                               "{} (\x1b[33m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               t,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }
                                    _ => {
                                        write!(f,
                                               "{} (\x1b[32m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               x /
                                               total_instructions.unwrap_or(std::f64::NAN) as f64 *
                                               100.,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }

                                }
                            }
                        }
                    }
                }


                Ok(())

            }

        }




    }
}
