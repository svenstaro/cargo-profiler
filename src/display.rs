use crate::profiler::Profiler;
use ndarray::Axis;
use std::fmt;

// pretty-print separator of functions
static DASHES: &'static str =
    "-----------------------------------------------------------------------";

/// Format a number with thousands separators. copied from cargo bench.
fn fmt_thousands_sep(n: f64, sep: char) -> String {
    let mut n_usize = n as usize;
    use std::fmt::Write;
    let mut output = String::new();
    let mut trailing = false;
    for &pow in &[9, 6, 3, 0] {
        let base = 10_usize.pow(pow);
        if pow == 0 || trailing || n_usize / base != 0 {
            if !trailing {
                output
                    .write_fmt(format_args!("{}", n_usize / base))
                    .unwrap();
            } else {
                output
                    .write_fmt(format_args!("{:03}", n_usize / base))
                    .unwrap();
            }
            if pow != 0 {
                output.push(sep);
            }
            trailing = true;
        }

        n_usize %= base;
    }

    output
}

/// Pretty-print the profiler outputs into user-friendly formats.
impl fmt::Display for Profiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Profiler::CacheGrind {
                ref ir,
                ref i1mr,
                ref ilmr,
                ref dr,
                ref d1mr,
                ref dlmr,
                ref dw,
                ref d1mw,
                ref dlmw,
                ref data,
                ref functs,
            } => {
                let _ = write!(
                    f,
                    "\n\x1b[32mTotal Memory Accesses\x1b[0m...{}\t\x1b[0m\n\
                       \n\x1b[32mTotal L1 I-Cache Misses\x1b[0m...{} ({}%)\t\x1b[0m\
                       \n\x1b[32mTotal LL I-Cache Misses\x1b[0m...{} ({}%)\t\x1b[0m\
                       \n\x1b[32mTotal L1 D-Cache Misses\x1b[0m...{} ({}%)\t\x1b[0m\
                        \n\x1b[32mTotal LL D-Cache Misses\x1b[0m...{} ({}%)\t\x1b[0m\n\n\

                    ",
                    fmt_thousands_sep(ir + dr + dw, ','),
                    fmt_thousands_sep(*i1mr, ','),
                    fmt_thousands_sep(i1mr / (ir + dr + dw) * 100., ','),
                    fmt_thousands_sep(*ilmr, ','),
                    fmt_thousands_sep(ilmr / (ir + dr + dw) * 100., ','),
                    fmt_thousands_sep(d1mr + d1mw, ','),
                    fmt_thousands_sep((d1mr + d1mw) / (ir + dr + dw) * 100., ','),
                    fmt_thousands_sep(dlmr + dlmw, ','),
                    fmt_thousands_sep((dlmr + dlmw) / (ir + dr + dw) * 100., ','),
                );
                let _ = write!(
                    f,
                    " \x1b[1;36mIr  \x1b[1;36mI1mr \x1b[1;36mILmr  \x1b[1;36mDr  \
                     \x1b[1;36mD1mr \x1b[1;36mDLmr  \x1b[1;36mDw  \x1b[1;36mD1mw \
                     \x1b[1;36mDLmw\n"
                );

                for (ref x, y) in data.axis_iter(Axis(0)).zip(functs.iter()) {
                    let _ = write!(
                        f,
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
                        y
                    );
                    println!("{}", DASHES);
                }
                Ok(())
            }

            Profiler::CallGrind {
                ref total_instructions,
                ref instructions,
                ref functs,
            } => {
                let _ = write!(
                    f,
                    "\n\x1b[32mTotal Instructions\x1b[0m...{}\n\n\x1b[0m",
                    fmt_thousands_sep(*total_instructions, ',')
                );

                for (&x, ref y) in instructions.iter().zip(functs.iter()) {
                    {
                        let perc = x / total_instructions * 100.;
                        match perc {
                            t if t >= 50.0 => {
                                let _ = write!(
                                    f,
                                    "{} (\x1b[31m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                    fmt_thousands_sep(x, ','),
                                    t,
                                    y
                                );
                                println!("{}", DASHES);
                            }
                            t if (t >= 30.0) & (t < 50.0) => {
                                let _ = write!(
                                    f,
                                    "{} (\x1b[33m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                    fmt_thousands_sep(x, ','),
                                    t,
                                    y
                                );
                                println!("{}", DASHES);
                            }
                            _ => {
                                let _ = write!(
                                    f,
                                    "{} (\x1b[32m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                    fmt_thousands_sep(x, ','),
                                    x / total_instructions * 100.,
                                    y
                                );
                                println!("{}", DASHES);
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fmt_thousands_sep() {
        assert_eq!(1, 1);
    }

}
