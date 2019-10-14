use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    for c in contents.chars() {
        match c {
            'T' => print!("U"),
            _ => print!("{}",c),
        }
    }
    println!();
}
