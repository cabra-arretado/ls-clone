use std::fs;

fn main() {
    list_files(&String::from("."));
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
