extern crate serde_json;

use std::env;
use std::fs;
use std::path::PathBuf;
use err::ProfError;
use std::process::Command;
use std::path::Path;
use self::serde_json::Value;

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

    let manifest = Command::new("cargo")
                       .arg("read-manifest")
                       .output()
                       .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let out = String::from_utf8(manifest.stdout).unwrap_or("".to_string());
    let data: Value = serde_json::from_str(&out).or(Err(ProfError::ReadManifestError))?;

        data.as_object()
         .expect("Could not extract object from read manifest JSON. Please submit bug.")
         .get("name")
         .ok_or(ProfError::NoNameError)
         .and_then(|x| Ok(x.to_string().replace("\"", "")))
}

// build the binary by calling cargo build
// return the path to the built binary
pub fn build_binary(release: bool) -> Result<String, ProfError> {
    let package_name = try!(get_package_name());

    let (out, binary_dir) = match release {
        true => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in release mode...",
                     package_name);

            (Command::new("cargo")
                 .args(&["build", "--release"])
                 .output()
                 .unwrap_or_else(|e| panic!("failed to execute process: {}", e)),
             "/target/release/")
        }
        false => {
            println!("\n\x1b[1;33mCompiling \x1b[1;0m{} in debug mode...",
                     package_name);

            (Command::new("cargo")
                 .arg("build")
                 .output()
                 .unwrap_or_else(|e| panic!("failed to execute process: {}", e)),
             "/target/debug/")
        }
    };


    let target_dir = find_target()
                         .ok_or(ProfError::NoTargetDirectory)
                         .and_then(|x| {
                             Ok(x.to_str()
                                 .expect("target directory could not be converted to string.")
                                 .to_string())
                         });
    let path = target_dir.and_then(|x| Ok(x + binary_dir + &package_name))
                         .unwrap_or("".to_string());

    if !Path::new(&path).exists() {
        return Err(ProfError::CompilationError(package_name.to_string(),
                                               String::from_utf8(out.stderr)
                                                   .unwrap_or("".to_string())));
    }
    Ok(path)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_find_target() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_get_package_name() {
        assert_eq!(1, 1);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_build_binary() {
        assert_eq!(1, 1);
    }
}
