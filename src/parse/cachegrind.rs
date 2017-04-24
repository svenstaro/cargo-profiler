extern crate ndarray;
extern crate regex;

use std::process::Command;
use self::ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};
use profiler::Profiler;
use std::cmp::Ordering::Less;
use err::ProfError;
use regex::Regex;
use std::ffi::OsStr;

/// initialize matrix object
pub type Mat<A> = OwnedArray<A, (Ix, Ix)>;

/// define cachegrind metrics
pub enum Metric {
    Ir,
    I1mr,
    ILmr,
    Dr,
    D1mr,
    DLmr,
    Dw,
    D1mw,
    DLmw,
    NAN,
}


/// Utility function for sorting a matrix. used to sort cachegrind data by particular metric (descending)
pub fn sort_matrix(mat: &Mat<f64>, sort_col: ArrayView<f64, Ix>) -> (Mat<f64>, Vec<usize>) {
    let mut enum_col = sort_col.iter().enumerate().collect::<Vec<(usize, &f64)>>();
    enum_col.sort_by(|a, &b| a.1.partial_cmp(b.1).unwrap_or(Less).reverse());
    let indices = enum_col.iter().map(|x| x.0).collect::<Vec<usize>>();
    (mat.select(Axis(0), indices.as_slice()), indices)
}


/// Parser trait. To parse the output of Profilers, we first have to get their output from
/// the command line, and then parse the output into respective structs.
pub trait CacheGrindParser {
    fn cachegrind_cli(&self, binary: &str, binargs: &[&OsStr]) -> Result<String, ProfError>;
    fn cachegrind_parse<'b>(&'b self,
                            output: &'b str,
                            num: usize,
                            sort_metric: Metric)
                            -> Result<Profiler, ProfError>;
}





impl CacheGrindParser for Profiler {
    /// Get profiler output from stdout.
    fn cachegrind_cli(&self, binary: &str, binargs: &[&OsStr]) -> Result<String, ProfError> {

        // get cachegrind cli output from stdout
        let _ = Command::new("valgrind")
                    .arg("--tool=cachegrind")
                    .arg("--cachegrind-out-file=cachegrind.out")
                    .arg(binary)
                    .args(binargs)
                    .output()
                    .or(Err(ProfError::CliError));

        let cachegrind_output = Command::new("cg_annotate")
                                    .arg("cachegrind.out")
                                    .arg(binary)
                                    .output()
                                    .or(Err(ProfError::CliError));

        cachegrind_output
            .and_then(|x| String::from_utf8(x.stdout).or(Err(ProfError::UTF8Error)))
            .or(Err(ProfError::CliError))
    }


    // Get parse the profiler output into respective structs.
    fn cachegrind_parse<'b>(&'b self,
                            output: &'b str,
                            num: usize,
                            sort_metric: Metric)
                            -> Result<Profiler, ProfError> {
        // split output line-by-line
        let mut out_split: Vec<&'b str> = output.split("\n").collect();

        // regex identifies lines that start with digits and have characters that commonly
        // show up in file paths
        lazy_static! {
           static ref CACHEGRIND_REGEX : Regex = Regex::new(r"\d+\s*[a-zA-Z]*$*_*:*/+\.*@*-*|\d+\s*[a-zA-Z]*$*_*\?+:*/*\.*-*@*-*").unwrap();
           static ref COMPILER_TRASH: Regex = Regex::new(r"\$\w{2}\$|\$\w{3}\$").unwrap();
           static ref ERROR_REGEX : Regex = Regex::new(r"Valgrind's memory management: out of memory").unwrap();
       }

        let errs = out_split.to_owned()
                            .into_iter()
                            .filter(|x| ERROR_REGEX.is_match(x))
                            .collect::<Vec<_>>();

        if errs.len() > 0 {
            return Err(ProfError::OutOfMemoryError);
        }

        out_split.retain(|x| CACHEGRIND_REGEX.is_match(x));

        let mut funcs: Vec<String> = Vec::new();
        let mut data_vec: Vec<Mat<f64>> = Vec::new();

        // loop through each line and get numbers + func
        for sample in out_split.iter() {

            println!("line: {}", sample);


            // trim the sample, split by whitespace to separate out each data point
            // (numbers + func)

            let mut elems = sample.trim()
                .split(" ")
                .collect::<Vec<&'b str>>();

            // remove any empty strings
            elems.retain(|x| x.to_string() != "");

            // for each number, remove any commas and parse into f64. the last element in
            // data_elems is the function file path.
            let mut numbers = Vec::new();

            for elem in elems[0..9].iter() {
                println!("  {}", elem);
                let number = match elem.trim().replace(",", "").parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => return Err(ProfError::RegexError),
                };

                numbers.push(number);
            }


            // reshape the vector of parsed numbers into a 1 x 9 matrix, and push the
            // matrix to our vector of 1 x 9 matrices.
            if let Ok(data_col) = OwnedArray::from_shape_vec((numbers.len(), 1), numbers) {
                data_vec.push(data_col);
            }
            // the elements after the 9 data_elems is the function file path.
            // get the file in the file-path (which includes the function) and push that to
            // the funcs vector.
            let first_char_of_path = sample.find(elems[9]).unwrap();
            let (_, function_path) = sample.split_at(first_char_of_path);
            let path = function_path.split("/")
                .collect::<Vec<&'b str>>();
            let func = path[path.len() - 1];

            let mut func = COMPILER_TRASH.replace_all(func, "");
            let idx = func.rfind("::").unwrap_or(func.len());
            func.drain(idx..).collect::<String>();
            funcs.push(func);

        }






        // stack all the 1 x 9 matrices in data to a n x 9  matrix.
        let data_matrix = match stack(Axis(1),
                                      &data_vec.iter()
                                               .map(|x| x.view())
                                               .collect::<Vec<_>>()
                                               .as_slice()) {
            Ok(m) => m.t().to_owned(),
            Err(_) => return Err(ProfError::MisalignedData),

        };


        // match the sort argument to a column of the matrix that we will sort on.
        // default sorting -> first column (total instructions).
        let sort_col = match sort_metric {
            Metric::Ir => data_matrix.column(0),
            Metric::I1mr => data_matrix.column(1),
            Metric::ILmr => data_matrix.column(2),
            Metric::Dr => data_matrix.column(3),
            Metric::D1mr => data_matrix.column(4),
            Metric::DLmr => data_matrix.column(5),
            Metric::Dw => data_matrix.column(6),
            Metric::D1mw => data_matrix.column(7),
            Metric::DLmw => data_matrix.column(8),
            Metric::NAN => data_matrix.column(0),
        };

        // sort the matrix of data and functions by a particular column.
        // to sort matrix, we keep track of sorted indices, and select the matrix wrt
        // these sorted indices. to sort functions, we index the funcs vector with the
        // sorted indices.
        let (mut sorted_data_matrix, indices) = sort_matrix(&data_matrix, sort_col);

        let mut sorted_funcs: Vec<String> = indices.iter()
                                                   .map(|&x| (&funcs[x]).to_owned())
                                                   .collect::<Vec<String>>();



        // sum the columns of the data matrix to get total metrics.
        let ir = sorted_data_matrix.column(0).scalar_sum();
        let i1mr = sorted_data_matrix.column(1).scalar_sum();
        let ilmr = sorted_data_matrix.column(2).scalar_sum();
        let dr = sorted_data_matrix.column(3).scalar_sum();
        let d1mr = sorted_data_matrix.column(4).scalar_sum();
        let dlmr = sorted_data_matrix.column(5).scalar_sum();
        let dw = sorted_data_matrix.column(6).scalar_sum();
        let d1mw = sorted_data_matrix.column(7).scalar_sum();
        let dlmw = sorted_data_matrix.column(8).scalar_sum();

        // parse the limit argument n, and take the first n values of data matrix/funcs
        // vector accordingly.
        if num < sorted_data_matrix.rows() {
            let ls = (0..num).collect::<Vec<_>>();
            sorted_data_matrix = sorted_data_matrix.select(Axis(0), ls.as_slice());
            sorted_funcs = sorted_funcs.iter()
                                       .take(num)
                                       .cloned()
                                       .collect();
        }



        // put all data in cachegrind struct!
        Ok(Profiler::CacheGrind {
            ir: ir,
            i1mr: i1mr,
            ilmr: ilmr,
            dr: dr,
            d1mr: d1mr,
            dlmr: dlmr,
            dw: dw,
            d1mw: d1mw,
            dlmw: dlmw,
            data: sorted_data_matrix,
            functs: sorted_funcs,
        })
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_cachegrind_parse_1() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_cachegrind_parse_2() {
        assert_eq!(1, 1);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_cachegrind_parse_3() {
        assert_eq!(1, 1);
    }
}
