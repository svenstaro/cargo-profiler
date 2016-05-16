use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use err::ProfError;
use regex::Regex;
/// Returns the closest ancestor path containing a `Cargo.toml`.
///
/// Returns `None` if no ancestor path contains a `Cargo.toml`, or if
/// the limit of MAX_ANCESTORS ancestors has been reached.
pub fn find_toml() -> Option<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path)
            .map(|entries| {
                entries.filter_map(|res| res.ok())
                       .any(|ent| &ent.file_name() == "Cargo.toml")
            })
            .unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir().ok().and_then(|mut wd| {
        for _ in 0..10 {
            if contains_manifest(&mut wd) {
                return Some(wd);
            }
            if !wd.pop() {
                break;
            }
        }

        None
    })
}

pub fn find_target() -> Option<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path)
            .map(|entries| {
                entries.filter_map(|res| res.ok())
                       .any(|ent| ent.path().ends_with("target"))
            })
            .unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir().ok().and_then(|mut wd| {
        for _ in 0..10 {
            if contains_manifest(&mut wd) {
                return Some(wd);
            }
            if !wd.pop() {
                break;
            }
        }

        None
    })
}

pub fn get_package_name(toml_dir: &PathBuf) -> Result<String, ProfError> {
    let toml = toml_dir.join("Cargo.toml");
    let mut f = try!(fs::File::open(toml));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let mut caps = Vec::new();

    lazy_static! {
       static ref PACKAGE_REGEX : Regex = Regex::new(r"\[package\]\n+name\s*=\s*.*").unwrap();
       static ref REPLACE_REGEX : Regex = Regex::new(r"\[package\]\n+name\s*=\s*").unwrap();
   }

    for cap in PACKAGE_REGEX.captures_iter(&s) {
        let r = REPLACE_REGEX.replace_all(cap.at(0).unwrap_or(""), "");
        let r = r.replace("\"", "");
        caps.push(r)

    }
    println!("{:?}", caps);
    Ok(caps[0].to_string())

}
