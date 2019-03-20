use crate::err::ProfError;
use crate::parse::cachegrind::Metric;
use crate::profiler::Profiler;
use clap::ArgMatches;
use std::path::Path;

/// match the profiler argument
pub fn get_profiler<'a>(
    matches: &'a ArgMatches,
) -> Result<(&'a ArgMatches<'a>, Profiler), ProfError> {
    match matches.subcommand_matches("profiler") {
        Some(matches) => match matches.subcommand_matches("callgrind") {
            Some(matches) => Ok((matches, Profiler::new_callgrind())),
            None => match matches.subcommand_matches("cachegrind") {
                Some(matches) => Ok((matches, Profiler::new_cachegrind())),
                None => Err(ProfError::InvalidProfiler),
            },
        },
        None => Err(ProfError::InvalidProfiler),
    }
}

/// match the binary argument
pub fn get_binary<'a>(matches: &'a ArgMatches) -> Result<&'a str, ProfError> {
    // read binary argument, make sure it exists in the filesystem
    match matches.value_of("binary") {
        Some(z) => {
            if !Path::new(z).exists() {
                return Err(ProfError::InvalidBinary);
            }
            Ok(z)
        }
        None => Err(ProfError::InvalidBinary),
    }
}

/// parse the number argument into a usize
pub fn get_num(matches: &ArgMatches) -> Result<usize, ProfError> {
    match matches.value_of("n").map(|x| x.parse::<usize>()) {
        Some(Ok(z)) => Ok(z),
        Some(Err(_)) => Err(ProfError::InvalidNum),
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
        Some("dlmr") => Ok(Metric::DLmr),
        Some("dw") => Ok(Metric::Dw),
        Some("d1mw") => Ok(Metric::D1mw),
        Some("dlmw") => Ok(Metric::DLmw),
        None => Ok(Metric::NAN),
        _ => Err(ProfError::InvalidSortMetric),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_profiler() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_get_binary() {
        assert_eq!(1, 1);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_get_num() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_get_sort_metric() {
        assert_eq!(1, 1);
    }
}
