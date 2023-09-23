use std::fs;
use std::env;

fn main() {
    let args = read_args();
    dbg!(&args);
    list_files(&args[1]);
}

fn list_files(folder_path: &str) {
    let paths = fs::read_dir(folder_path).unwrap();
    for path in paths {
        if path.is_err() {
            continue;
        }
        //TODO: Remove the main path from the string
        println!("{}", path.unwrap().path().display());
    }
}

fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}
