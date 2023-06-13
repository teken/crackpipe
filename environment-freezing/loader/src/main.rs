use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("test/file.txt").unwrap();

    println!("{}", contents);
}
