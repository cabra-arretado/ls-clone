use std::env;
use std::fs;

fn main() {
    let args = read_args();
    // dbg!(&args);
    list_files(&args[1]);
}

fn list_files(folder_path: &str) {
    let paths = fs::read_dir(folder_path);
    match paths {
        Err(_) => println!("Error reading the path"),
        Ok(_) => {
            for path in paths.unwrap() {
                if path.is_err() {
                    continue;
                }
                println!("{}", path.unwrap().path().display());
            }
        }
    }
}

fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}
