// build.rs

use std::process::Command;
use std::path::Path;
use std::process;
use std::str;


fn main() {
    if !Path::new("..\\gcc_wrapper\\target\\debug\\gcc_wrapper.exe").exists() {
        let output = Command::new("cargo")
            .args(&["build"])
            .current_dir("..\\gcc_wrapper")
            .output()
            .expect("failed to execute process");

        let out = str::from_utf8(&(output.stdout)).expect("unable to get stdout");
        eprintln!("{}", out);

        let err_out = str::from_utf8(&(output.stderr)).expect("unable to get stderr");
        eprintln!("{}", err_out);

        match output.status.code() {
            Some(code) => process::exit(code),
            None       => process::exit(42)
        }
    }
 }