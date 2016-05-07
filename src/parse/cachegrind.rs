extern crate ndarray;
use std::process::Command;
use self::ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};
use profiler::Profiler;
use std::f64;
use std::cmp::Ordering::Less;
// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;


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
    NAN,
}


// utility function for sorting a matrix. used to sort cachegrind data by particular metric (descending)
pub fn sort_matrix(mat: &Mat<f64>, sort_col: ArrayView<f64, Ix>) -> (Mat<f64>, Vec<usize>) {
    let mut enum_col = sort_col.iter().enumerate().collect::<Vec<(usize, &f64)>>();
    enum_col.sort_by(|a, &b| a.1.partial_cmp(b.1).unwrap_or(Less).reverse());
    let indices = enum_col.iter().map(|x| x.0).collect::<Vec<usize>>();
    (mat.select(Axis(0), indices.as_slice()), indices)
}


// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait CacheGrindParser {
    fn cachegrind_cli(&self, binary: &str) -> String;
    fn cachegrind_parse<'b>(&'b self, output: &'b str, num: usize, s: Metric) -> Profiler;
}





impl<'a> CacheGrindParser for Profiler<'a> {
    // Get profiler output from stdout.
    fn cachegrind_cli(&self, binary: &str) -> String {

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
    // Get parse the profiler output into respective structs.
    fn cachegrind_parse<'b>(&'b self,
                            output: &'b str,
                            num: usize,
                            sort_metric: Metric)
                            -> Profiler {
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
                             .map(|x|
                             {
                                  match x.trim().replace(",", "").parse::<f64>(){
                                      Ok(rep) => rep,
                                      Err(rep) => panic!("regex problem at cachegrind output, failed at number {}", rep)
                                }
                            })
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
        let mat = match stack(Axis(1),
                        &data.iter().map(|x| x.view()).collect::<Vec<_>>().as_slice())
                    {
                        Ok(m) => m,
                        Err(_) => panic!("data arrays are misaligned")
                    };

        // transpose the matrix so we have a n x 9 matrix displayed.
        // let mat = mat.t();

        // match the sort argument to a column of the matrix that we will sort on.
        // default sorting -> first column (total instructions).
        let sort_col = match sort_metric {
            Metric::Ir => mat.column(0),
            Metric::I1mr => mat.column(1),
            Metric::Ilmr => mat.column(2),
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
        let (mut sorted_mat, indices) = sort_matrix(&mat, sort_col);
        let mut sorted_funcs = indices.iter().map(|&x| funcs[x]).collect::<Vec<&'b str>>();


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
            data: sorted_mat,
            functs: Some(sorted_funcs),
        }
    }
}
