use std::fs;

fn main() {
    list_files(".");
}

fn list_files(folder_path: &str) {
    let paths = fs::read_dir(folder_path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
