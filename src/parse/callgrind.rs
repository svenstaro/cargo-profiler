extern crate ndarray;
use std::process::Command;
use profiler::Profiler;
use std::f64;

// Parser trait. To parse the output of Profilers, we first have to get their output from
// the command line, and then parse the output into respective structs.
pub trait CallGrindParser {
    fn callgrind_cli(&self, binary: &str) -> String;
    fn callgrind_parse<'b>(&'b self, output: &'b str, num: usize) -> Profiler;
}


impl<'a> CallGrindParser for Profiler<'a> {
    // Get profiler output from stdout.
    fn callgrind_cli(&self, binary: &str) -> String {


        // get callgrind cli output from stdout
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

    fn callgrind_parse<'b>(&'b self, output: &'b str, num: usize) -> Profiler {

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
            let number = match elems[0].trim().replace(",", "").parse::<f64>(){
                Ok(rep) => rep,
                Err(rep) => panic!("regex problem at callgrind output, failed at number {}", rep)
          };

            data.push(number);


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
        Profiler::CallGrind {
            total_instructions: total_instructions,
            instructions: data,
            functs: Some(funcs),
        }
    }
}
