use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use err::ProfError;
use regex::Regex;
use std::process::{Command, exit};
use std::path::Path;

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

/// Returns the closest ancestor path containing a `target` directory.
///
/// Returns `None` if no ancestor path contains a `Cargo.toml`, or if
/// the limit of MAX_ANCESTORS ancestors has been reached.
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


// returns the name of the package parsed from Cargo.toml
// this will only work if the package name is directly underneath [package] tag
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
    let captures_iter = PACKAGE_REGEX.captures_iter(&s);
    if captures_iter.collect::<Vec<_>>().len() == 0 {
        println!("{}", ProfError::TomlError);
        exit(1);
    }
    for cap in PACKAGE_REGEX.captures_iter(&s) {

        let c = cap.at(0).unwrap_or("");
        let r = REPLACE_REGEX.replace_all(c, "");
        let r = r.replace("\"", "");
        caps.push(r)

    }
    Ok(caps[0].to_string())

}

// build the binary by calling cargo build
// return the path to the built binary
pub fn build_binary(release: bool, package_name: &str) -> Result<String, ProfError> {

    match release {
        true => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in release mode...",
                     package_name);
            let _ = Command::new("cargo")
                        .args(&["build", "--release"])
                        .output();
            let target_dir = find_target().unwrap().to_str().unwrap().to_string();
            let path = target_dir + "/target/release/" + &package_name;
            if !Path::new(&path).exists() {
                println!("{}", ProfError::CompilationError(package_name.to_string()));
                exit(1);

            }
            return Ok(path);

        }
        false => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in debug mode...",
                     package_name);
            let _ = Command::new("cargo")
                        .arg("build")
                        .output()
                        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));;
            let target_dir = find_target().unwrap().to_str().unwrap().to_string();
            let path = target_dir + "/target/debug/" + &package_name;
            if !Path::new(&path).exists() {
                println!("{}", ProfError::CompilationError(package_name.to_string()));
                exit(1);

            }
            return Ok(path);
        }

    }
}
