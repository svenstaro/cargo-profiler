extern crate ndarray;

use std::process::Command;
use std::fmt;
use self::ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};
use std::f64;
// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;


// utility function for sorting a matrix. used to sort cachegrind data by particular metric.
pub fn sort_matrix(mat : &Mat<f64>, sort_col: ArrayView<f64,Ix>) -> (Mat<f64>, Vec<usize>){
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

pub struct CacheGrind<'a> {
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
}

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait CacheGrindParser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str,num: usize, s: Metric) -> CacheGrind;
}



impl <'a> CacheGrind<'a>{
    pub fn new_cachegrind() -> CacheGrind<'a> {
        CacheGrind {
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
            data: OwnedArray::from_shape_vec((10, 9), vec![f64::NAN; 10 *9]).ok().unwrap(),
            // profiled functions in binary
            functs: None,
        }
    }
}

impl<'a> CacheGrindParser for CacheGrind<'a> {
    fn cli(&self, binary: &str) -> String {
            // get cachegrind cli output from stdout
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

    fn parse<'b>(&'b self, output: &'b str, num: usize, sort_metric : Metric) -> CacheGrind {
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
        // let mat = mat.t();

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
            Metric::NAN => mat.column(0),
        };

        // sort the matrix of data and functions by a particular column.
        // to sort matrix, we keep track of sorted indices, and select the matrix wrt
        // these sorted indices. to sort functions, we index the funcs vector with the
        // sorted indices.
        let (mut sorted_mat, indices) = sort_matrix(&mat,sort_col);
        let mut sorted_funcs = indices.iter().map(|&x| funcs[x]).collect::<Vec<&'b str>>();


        // also, when we sort, we want to make sure that we reverse the order of the
        // matrix/funcs so that the most expensive functions show up at the top.

        let mut reverse_indices = (0..sorted_mat.rows()).collect::<Vec<usize>>();
        reverse_indices.reverse();
        sorted_mat = sorted_mat.select(Axis(0), reverse_indices.as_slice());
        &sorted_funcs.reverse();

        // sum the columns of the data matrix to get total metrics.
        let ir = sorted_mat.column(0).scalar_sum();
        let i1mr = sorted_mat.column(1).scalar_sum();
        let ilmr = sorted_mat.column(2).scalar_sum();
        let dr = sorted_mat.column(3).scalar_sum();
        let d1mr = sorted_mat.column(4).scalar_sum();
        let dlmr = sorted_mat.column(5).scalar_sum();
        let dw = sorted_mat.column(6).scalar_sum();
        let d1mw = sorted_mat.column(7).scalar_sum();
        let dlmw = sorted_mat.column(8).scalar_sum();


        // parse the limit argument n, and take the first n values of data matrix/funcs
        // vector accordingly.
        if num < sorted_mat.rows() {
            let ls = (0..num).collect::<Vec<_>>();

            sorted_mat = sorted_mat.select(Axis(0), ls.as_slice());

            sorted_funcs = sorted_funcs.iter()
                                       .take(num)
                                       .cloned()
                                       .collect();
        }



        // put all data in cachegrind struct!
        CacheGrind {
            ir: ir,
            i1mr: i1mr,
            ilmr: ilmr,
            dr: dr,
            d1mr: d1mr,
            dlmr: dlmr,
            dw: dw,
            d1mw: d1mw,
            dlmw: dlmw,
            data: sorted_mat,
            functs: Some(sorted_funcs),
        }
    }
}

impl<'a> fmt::Display for CacheGrind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
               self.ir,
               self.i1mr,
               self.ilmr,
               self.dr,
               self.d1mr,
               self.dlmr,
               self.dw,
               self.d1mw,
               self.dlmw,
           );
        write!(f,
               " \x1b[1;36mIr  \x1b[1;36mI1mr \x1b[1;36mILmr  \x1b[1;36mDr  \
                \x1b[1;36mD1mr \x1b[1;36mDLmr  \x1b[1;36mDw  \x1b[1;36mD1mw \
                \x1b[1;36mDLmw\n");

        if let Some(ref func) = self.functs {
                for (ref x, &y) in self.data.axis_iter(Axis(0)).zip(func.iter()) {
                    write!(f,
                           "\x1b[0m{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} \
                            {}\n",
                           x[0] / self.ir,
                           x[1] / self.i1mr,
                           x[2] / self.ilmr,
                           x[3] / self.dr,
                           x[4] / self.d1mr,
                           x[5] / self.dlmr,
                           x[6] / self.dw,
                           x[7] / self.d1mw,
                           x[8] / self.dlmw,
                           y);
                    println!("-----------------------------------------------------------------------");


                }

        }
        Ok(())
    }
}
