extern crate ndarray;

use std::fmt;
use profiler::Profiler;
use self::ndarray::Axis;
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
                               "\x1b[0m{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {}\n",
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

                            let perc = x / total_instructions * 100.;
                            match perc {
                                t if t >= 50.0 => {
                                    write!(f, "{} (\x1b[31m{:.1}%\x1b[0m)\x1b[0m {}\n", x, t, y);
                                    println!("-----------------------------------------------------------------------");
                                }
                                t if (t >= 30.0) & (t < 50.0) => {
                                    write!(f, "{} (\x1b[33m{:.1}%\x1b[0m)\x1b[0m {}\n", x, t, y);
                                    println!("-----------------------------------------------------------------------");
                                }
                                _ => {
                                    write!(f,
                                           "{} (\x1b[32m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                           x,
                                           x / total_instructions * 100.,
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
