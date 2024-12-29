use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);
    let contents = fs::read_to_string(file_path).unwrap();

    println!("{contents}");
}
