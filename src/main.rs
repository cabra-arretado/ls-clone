extern crate getopts;

use std::env;
use std::fs;

fn main() {
    let args = read_args();
    let program = &args[0];

    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        std::process::exit(0);
    }

    list_files(&args[1]);
}

fn list_files(folder_path: &str) {
    let files = get_files_vec(folder_path);
    for file in files {
        print!("{} ", file);
    }
    println!("");
    std::process::exit(0);
}

fn get_files_vec(folder_path: &str) -> Vec<String> {
    let paths = fs::read_dir(folder_path);
    match paths {
        Err(_) => { 
            println!("Error: reading the path");
            std::process::exit(1);
        },
        Ok(_) => {
            let mut files: Vec<String> = Vec::new();
            for path in paths.unwrap() {
                if path.is_err() {
                    continue;
                }
                files.push(path.unwrap().path().display().to_string());
            }
            files
        }
    }

}

fn read_args() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Error: missing arguments");
        std::process::exit(1);
    }
    args[..].to_vec()
}

fn print_usage(opts: getopts::Options) {
    let program = "ls-clone";
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
