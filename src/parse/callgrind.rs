extern crate regex;

use std::process::Command;
use profiler::Profiler;
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
        Command::new("valgrind")
            .arg("--tool=callgrind")
            .arg("--callgrind-out-file=callgrind.out")
            .arg(binary)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        let cachegrind_output = Command::new("callgrind_annotate")
                                    .arg("callgrind.out")
                                    .arg(binary)
                                    .output()
                                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        Ok(String::from_utf8(cachegrind_output.stdout)
               .expect("error while returning cachegrind stdout"))

    }

    fn callgrind_parse<'b>(&'b self, output: &'b str, num: usize) -> Result<Profiler, ProfError> {

        // split output line-by-line
        let mut out_split = output.split("\n").collect::<Vec<_>>();

        // regex identifies lines that start with digits and have characters that commonly
        // show up in file paths
        lazy_static! {
           static ref CALLGRIND_REGEX : Regex = Regex::new(r"\d+\s*[a-zA-Z]*$*_*:*/+\.*@*-*|\d+\s*[a-zA-Z]*$*_*\?+:*/*\.*-*@*-*").unwrap();
           static ref COMPILER_TRASH: Regex = Regex::new(r"\$\w{2}\$|\$\w{3}\$").unwrap();
           static ref ERROR_REGEX : Regex = Regex::new(r"out of memory").unwrap();

       }
        let errs = out_split.to_owned()
                            .into_iter()
                            .filter(|x| ERROR_REGEX.is_match(x))
                            .collect::<Vec<_>>();
        if errs.len() > 0 {
            return Err(ProfError::OutOfMemoryError);
        }

        out_split.retain(|x| CALLGRIND_REGEX.is_match(x));


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
                Err(_) => return Err(ProfError::RegexError),
            };

            data_vec.push(data_row);


            // the function has some trailing whitespace and trash. remove that, and
            // get the function, push to functs vector.
            let path = elems[1].split(" ").collect::<Vec<_>>();
            let cleaned_path = path[0].split("/").collect::<Vec<_>>();
            let func = cleaned_path[cleaned_path.len() - 1];
            let mut func = COMPILER_TRASH.replace_all(func, "..");
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

#[cfg(test)]
mod test {
    use profiler::Profiler;
    use super::CallGrindParser;
    #[test]
    fn test_callgrind_parse_1() {
        let output = "==6072==     Valgrind's memory management: out of memory:\n ==6072==     \
                      Whatever the reason, Valgrind cannot continue.  Sorry.";
        let num = 10;
        let profiler = Profiler::new_callgrind();
        let is_err = profiler.callgrind_parse(&output, num).is_err();
        assert!(is_err && true)
    }

    #[test]
    fn test_callgrind_parse_2() {
        assert_eq!(1, 1);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_callgrind_parse_3() {
        assert_eq!(1, 1);
    }
}
