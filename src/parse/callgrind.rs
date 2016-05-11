extern crate ndarray;
extern crate regex;

use std::process::Command;
use profiler::Profiler;
use std::f64;
use err::ProfError;
use regex::Regex;

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait CallGrindParser {
    fn callgrind_cli(&self, binary: &str) -> Result<String, ProfError>;
    fn callgrind_parse<'b>(&'b self, output: &'b str, num: usize) -> Result<Profiler, ProfError>;
}


impl CallGrindParser for Profiler {
    // Get profiler output from stdout.
    fn callgrind_cli(&self, binary: &str) -> Result<String, ProfError> {

        // get callgrind cli output from stdout
        try!(Command::new("valgrind")
                 .arg("--tool=callgrind")
                 .arg("--callgrind-out-file=callgrind.out")
                 .arg(binary)
                 .output());

        let cachegrind_output = try!(Command::new("callgrind_annotate")
                                         .arg("callgrind.out")
                                         .arg(binary)
                                         .output());
        Ok(String::from_utf8(cachegrind_output.stdout)
               .expect("error while returning cachegrind stdout"))

    }

    fn callgrind_parse<'b>(&'b self, output: &'b str, num: usize) -> Result<Profiler, ProfError> {

        // split output line-by-line
        let mut out_split = output.split("\n").collect::<Vec<_>>();

        // regex identifies lines that start with digits and have characters that commonly
        // show up in file paths
        lazy_static! {
           static ref callgrind_regex : Regex = Regex::new(r"\d+\s*[a-zA-Z]*$*_*:*/+\.*@*-*|\d+\s*[a-zA-Z]*$*_*\?+:*/*\.*-*@*-*").unwrap();
           static ref compiler_trash: Regex = Regex::new(r"\$\w{2}\$|\$\w{3}\$").unwrap();

       }

        out_split.retain(|x| callgrind_regex.is_match(x));


        let mut funcs: Vec<String> = Vec::new();
        let mut data_vec: Vec<f64> = Vec::new();
        // loop through each line and get numbers + func
        for sample in out_split.iter() {

            // trim the sample, split by whitespace to separate out each data point
            // (numbers + func)
            let elems = sample.trim().split("  ").collect::<Vec<_>>();

            // for each number, remove any commas and parse into f64. the last element in
            // data_elems is the function file path.
            let data_row = match elems[0].trim().replace(",", "").parse::<f64>() {
                Ok(rep) => rep,
                Err(rep) => {
                    panic!("regex problem at callgrind output, failed at value {}. Please file a \
                            bug.",
                           rep)
                }
            };

            data_vec.push(data_row);


            // the function has some trailing whitespace and trash. remove that, and
            // get the function, push to functs vector.
            let path = elems[1].split(" ").collect::<Vec<_>>();
            let cleaned_path = path[0].split("/").collect::<Vec<_>>();
            let func = cleaned_path[cleaned_path.len() - 1];
            let mut func = compiler_trash.replace_all(func, "..");
            let idx = func.rfind("::").unwrap_or(func.len());
            func.drain(idx..).collect::<String>();
            funcs.push(func)

        }

        // get the total instructions by summing the data vector.
        let total_instructions = data_vec.iter().fold(0.0, |a, b| a + b);

        // parse the limit argument n, and take the first n values of data/funcs vectors
        // accordingly.

        if num < data_vec.len() {
            data_vec = data_vec.iter().take(num).cloned().collect();
            funcs = funcs.iter().take(num).cloned().collect();
        }
        // put all data in cachegrind struct!
        Ok(Profiler::CallGrind {
            total_instructions: total_instructions,
            instructions: data_vec,
            functs: funcs,
        })
    }
}
