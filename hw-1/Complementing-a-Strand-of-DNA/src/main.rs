use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    for c in contents.chars().rev() {
        match c {
            'A' => print!("T"),
            'T' => print!("A"),
            'C' => print!("G"),
            'G' => print!("C"),
            _ => print!("{}", c),
        }
    }
    println!();
}
