extern crate clap;
use clap::ArgMatches;
use profiler::Profiler;
use parse::cachegrind::Metric;
use err::ProfError;
use std::path::Path;
use std::process;

/// match the profiler argument
pub fn get_profiler<'a>(matches: &'a ArgMatches)
                        -> Result<(&'a ArgMatches<'a>, Profiler), ProfError> {
    match matches.subcommand_matches("profiler") {
        Some(matches) => {
            match matches.subcommand_matches("callgrind") {
                Some(matches) => Ok((matches, Profiler::new_callgrind())),
                None => {
                    match matches.subcommand_matches("cachegrind") {
                        Some(matches) => Ok((matches, Profiler::new_cachegrind())),
                        None => {
                            println!("{}", ProfError::InvalidProfiler);
                            process::exit(1);
                        }
                    }
                }
            }
        }
        None => {
            println!("{}", ProfError::InvalidProfiler);
            process::exit(1);
        }
    }
}

/// match the binary argument
pub fn get_binary<'a>(matches: &'a ArgMatches) -> Result<&'a str, ProfError> {
    // read binary argument, make sure it exists in the filesystem
    match matches.value_of("binary") {
        Some(z) => {
            if !Path::new(z).exists() {
                println!("{}", ProfError::InvalidBinary);
                process::exit(1);

            }
            return Ok(z);
        }
        None => {
            println!("{}", ProfError::InvalidBinary);
            process::exit(1);
        }
    }


}

/// parse the number argument into a usize
pub fn get_num(matches: &ArgMatches) -> Result<usize, ProfError> {

    match matches.value_of("n").map(|x| x.parse::<usize>()) {
        Some(Ok(z)) => Ok(z),
        Some(Err(_)) => {
            println!("{}", ProfError::InvalidNum);
            process::exit(1);
        }
        None => Ok(10000), // some arbitrarily large number...
    }

}

/// get the cachegrind metric user wants to sort on
pub fn get_sort_metric(matches: &ArgMatches) -> Result<Metric, ProfError> {
    match matches.value_of("sort") {
        Some("ir") => Ok(Metric::Ir),
        Some("i1mr") => Ok(Metric::I1mr),
        Some("ilmr") => Ok(Metric::ILmr),
        Some("dr") => Ok(Metric::Dr),
        Some("d1mr") => Ok(Metric::D1mr),
        Some("llmr") => Ok(Metric::LLmr),
        Some("dw") => Ok(Metric::Dw),
        Some("d1mw") => Ok(Metric::D1mw),
        Some("llmw") => Ok(Metric::LLmw),
        None => Ok(Metric::NAN),
        _ => {
            println!("{}", ProfError::InvalidSortMetric);
            process::exit(1);
        }

    }
}
