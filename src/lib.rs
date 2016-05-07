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

pub enum Metric {
    Ir,
    I1mr,
    Ilmr,
    Dr,
    D1mr,
    Dlmr,
    Dw,
    D1mw,
    Dlmw,
    NAN
}

// Profiler enum. We have two profilers: CacheGrind and CallGrind.
pub enum Profiler<'a> {
    // CachGrind holds the parsed objects of
    // `valgrind --tool=cachegrind -cachegrind-out-file=cachegrind.out && cg_annotate
    // cachegrind.out`
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
    // `valgrind --tool=callgrind --callgrind-out-file=callgrind.out && callgrind_annotate
    // callgrind.out`
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
            ir: std::f64::NAN,
            // total i1-cache read misses
            i1mr: std::f64::NAN,
            // total iL-cache read misses
            ilmr: std::f64::NAN,
            // total reads
            dr: std::f64::NAN,
            // total d1-cache read misses
            d1mr: std::f64::NAN,
            // total dL-cache read misses
            dlmr: std::f64::NAN,
            // total d-cache writes
            dw: std::f64::NAN,
            // total d1-cache write-misses
            d1mw: std::f64::NAN,
            // total dL cache write misses
            dlmw: std::f64::NAN,
            // profiler data
            data: OwnedArray::from_shape_vec((10, 9), vec![std::f64::NAN; 10 *9]).ok().unwrap(),
            // profiled functions in binary
            functs: None,
        }
    }
    // Initialize CallGrind
    pub fn new_callgrind() -> Profiler<'a> {
        Profiler::CallGrind {
            // total instruction calls
            total_instructions: std::f64::NAN,
            // instruction data
            instructions: vec![0.],
            // profiled functions in binary
            functs: None,
        }
    }
}

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait Parser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str,n: &str, s: Metric) -> Profiler;
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
    fn parse<'b>(&'b self, output: &'b str, n: &str, sort_metric : Metric) -> Profiler {
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
                                      .map(|x| x.trim().replace(",", "").parse::<f64>().ok().unwrap())
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
                              .ok().unwrap()
                              ;
                // transpose the matrix so we have a n x 9 matrix displayed.
                let mat = mat.t();

                // match the sort argument to a column of the matrix that we will sort on.
                // default sorting -> first column (total instructions).
                let sort_col = match sort_metric {
                    Metric::Ir => mat.column(0),
                    Metric::I1mr => mat.column(1),
                    Metric::Ilmr  => mat.column(2),
                    Metric::Dr => mat.column(3),
                    Metric::D1mr => mat.column(4),
                    Metric::Dlmr => mat.column(5),
                    Metric::Dw => mat.column(6),
                    Metric::D1mw => mat.column(7),
                    Metric::Dlmw => mat.column(8),
                    _ => panic!("sort argument is not valid"),
                };

                // sort the matrix of data and functions by a particular column.
                // to sort matrix, we keep track of sorted indices, and select the matrix wrt
                // these sorted indices. to sort functions, we index the funcs vector with the
                // sorted indices.
                let (mut sorted_funcs, mut mat) = match sort_metric {
                    Metric::NAN => (funcs, mat.to_owned()),
                    _ => {
                        let (mat, indices) = sort_matrix(mat.to_owned(),sort_col);
                        (indices.iter().map(|&x| funcs[x]).collect::<Vec<&'b str>>(), mat)

                    }
                };

                // also, when we sort, we want to make sure that we reverse the order of the
                // matrix/funcs so that the most expensive functions show up at the top.
                match sort_metric {
                    Metric::NAN => {}
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
                    ir: ir,
                    i1mr: i1mr,
                    ilmr: ilmr,
                    dr: dr,
                    d1mr: d1mr,
                    dlmr: dlmr,
                    dw: dw,
                    d1mw: d1mw,
                    dlmw: dlmw,
                    data: mat,
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
                    total_instructions:total_instructions,
                    instructions: data,
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
                       ir,
                       i1mr,
                       ilmr,
                       dr,
                       d1mr,
                       dlmr,
                       dw,
                       d1mw,
                       dlmw,
                   );
                write!(f,
                       " \x1b[1;36mIr  \x1b[1;36mI1mr \x1b[1;36mILmr  \x1b[1;36mDr  \
                        \x1b[1;36mD1mr \x1b[1;36mDLmr  \x1b[1;36mDw  \x1b[1;36mD1mw \
                        \x1b[1;36mDLmw\n");

                if let &Some(ref func) = functs {
                        for (ref x, &y) in data.axis_iter(Axis(0)).zip(func.iter()) {
                            write!(f,
                                   "\x1b[0m{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} \
                                    {}\n",
                                   x[0] / ir,
                                   x[1] / i1mr,
                                   x[2] / ilmr,
                                   x[3] / dr,
                                   x[4] / d1mr,
                                   x[5] / dlmr,
                                   x[6] / dw,
                                   x[7] / d1mw,
                                   x[8] / dlmw,
                                   y);
                            println!("-----------------------------------------------------------------------");


                        }

                }
                Ok(())
            }

            Profiler::CallGrind { ref total_instructions, ref instructions, ref functs } => {

                write!(f,
                       "\n\x1b[32mTotal Instructions\x1b[0m...{}\n\n\x1b[0m",
                       total_instructions);

                if let &Some(ref func) = functs {
                        for (&x, &y) in instructions.iter().zip(func.iter()) {
                            {

                                let perc = x / total_instructions *
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
                                               total_instructions *
                                               100.,
                                               y);
                                        println!("-----------------------------------------------------------------------");
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
