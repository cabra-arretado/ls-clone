use getopts::Options;

use std::env;
use std::fs;

fn main() {
    let args = read_args();
    // let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("a", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        println!("{}", String::from("Help me"));
        return;
    }
    list_files(&args[0]);
}

fn list_files(folder_path: &str) {
    let paths = fs::read_dir(folder_path);
    match paths {
        Err(_) => println!("Error: reading the path"),
        Ok(_) => {
            for path in paths.unwrap() {
                if path.is_err() {
                    continue;
                }
                println!("{}", path.unwrap().path().display());
                std::process::exit(0);
            }
        }
    }
}

fn read_args() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Error: missing arguments");
        std::process::exit(1);
    }
    args[1..].to_vec()
}
