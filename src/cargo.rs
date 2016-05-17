extern crate rustc_serialize;

use std::env;
use std::fs;
use std::path::PathBuf;
use err::ProfError;
use std::process::{Command, exit};
use std::path::Path;
use self::rustc_serialize::json::Json;

/// Returns the closest ancestor path containing a `target` directory.
///
/// Returns `None` if no ancestor path contains a `target` directory, or if
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
pub fn get_package_name() -> Result<String, ProfError> {

    let manifest = match Command::new("cargo").arg("read-manifest").output() {
        Ok(m) => m,
        Err(_) => {
            println!("{}", ProfError::ReadManifestError);
            exit(1);
        }
    };


    let data = Json::from_str(&String::from_utf8(manifest.stdout)
                                   .expect("Error while returning manifest JSON stdout"))
                   .expect("Error in encoding manifest string into JSON");


    match data.as_object().expect("Could not extract Object from JSON").get("name") {

        Some(n) => Ok(n.to_string().replace("\"", "")),
        None => {
            println!("{}", ProfError::NoNameError);
            exit(1);
        }
    }



}

// build the binary by calling cargo build
// return the path to the built binary
pub fn build_binary(release: bool, package_name: &str) -> Result<String, ProfError> {

    match release {
        true => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in release mode...",
                     package_name);
            let out = Command::new("cargo")
                          .args(&["build", "--release"])
                          .output()
                          .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
            let target_dir = find_target().unwrap().to_str().unwrap().to_string();
            let path = target_dir + "/target/release/" + &package_name;
            if !Path::new(&path).exists() {
                println!("{}",
                         ProfError::CompilationError(package_name.to_string(),
                                                     String::from_utf8(out.stderr).unwrap()));
                exit(1);

            }
            return Ok(path);

        }
        false => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in debug mode...",
                     package_name);
            let out = Command::new("cargo")
                          .arg("build")
                          .output()
                          .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
            let target_dir = find_target().unwrap().to_str().unwrap().to_string();
            let path = target_dir + "/target/debug/" + &package_name;
            if !Path::new(&path).exists() {
                println!("{}",
                         ProfError::CompilationError(package_name.to_string(),
                                                     String::from_utf8(out.stderr).unwrap()));
                exit(1);

            }
            return Ok(path);
        }

    }
}
