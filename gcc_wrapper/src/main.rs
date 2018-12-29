use std::env;
use std::process::Command;
use std::process;
use std::str;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut args_to_gcc: Vec<String> = Vec::new();
    let mut idx = 0;
    let mut take_next = false;

    if args[1].starts_with("@") {
        let filename = &args[1][1..];
        let file = File::open(filename).expect("unable to open linker args file");
        let buf = BufReader::new(file);
        let mut lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        args.append(&mut lines);
    }

    for arg in args {
        if idx == 0 {
            idx = idx + 1;
            continue;
        }

        if take_next {
            take_next = false;
            args_to_gcc.push(arg.clone());
        }

        if arg == "-L" {
            args_to_gcc.push(arg.clone());
            take_next = true;
            idx = idx +1;
            continue;
        }

        if arg == "-o" {
            args_to_gcc.push(arg.clone());
            take_next = true;
            idx = idx +1;
            continue;
        }

        if arg.ends_with(".o") {
            args_to_gcc.push(arg.clone());
        }

        if arg.ends_with(".rlib") {
            args_to_gcc.push(arg.clone());
        }

        idx = idx +1;
    };

    args_to_gcc.push("--sysroot=C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1+Beta1811".to_string());
    args_to_gcc.push("-Wl,--no-undefined,--gc-sections".to_string());
    args_to_gcc.push("-nodefaultlibs".to_string());
    args_to_gcc.push("-B".to_string());
    args_to_gcc.push("C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1+Beta1811\\tools\\gcc".to_string());
    args_to_gcc.push("-march=armv7ve".to_string());
    args_to_gcc.push("-mcpu=cortex-a7".to_string());
    args_to_gcc.push("-mthumb".to_string());
    args_to_gcc.push("-mfpu=neon".to_string());
    args_to_gcc.push("-mfloat-abi=hard".to_string());
    args_to_gcc.push("-lapplibs".to_string());
    args_to_gcc.push("-lpthread".to_string());
    args_to_gcc.push("-lgcc_s".to_string());
    args_to_gcc.push("-lc".to_string());
    args_to_gcc.push("-Os".to_string());

    eprintln!("args to gcc {:?}", args_to_gcc);

    let output =  Command::new("C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1+Beta1811\\tools\\gcc\\gcc.exe")
            .args(&args_to_gcc)
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
