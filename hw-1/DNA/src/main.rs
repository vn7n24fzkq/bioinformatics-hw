use std::env;
use std::fs;

fn main() {
    let mut a_count = 0;
    let mut c_count = 0;
    let mut g_count = 0;
    let mut t_count = 0;
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    for c in contents.chars() {
        match c {
            'A' => a_count += 1,
            'C' => c_count += 1,
            'G' => g_count += 1,
            'T' => t_count += 1,
            _ => println!("I don't know this DNA{}", c),
        }
    }

    println!("A:{} C:{} G:{} T:{}", a_count,c_count,g_count,t_count);
}
