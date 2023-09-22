use std::fs;

fn main() {
    list_files();
}

fn list_files() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
