use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number = &args[1];
    let filename = &args[2];

    println!(
        "Searching for two values in {} that sum to {}",
        filename, number
    );

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers_strings = contents.split("\n");

    let numbers: Vec<i64> = numbers_strings
        .map(|s| s.parse().expect("Parse Numbers"))
        .collect();

    for n in &numbers {
        for n2 in &numbers {
            if n + n2 == 2020 {
                println!("{0}+{1}={2}, {0}*{1}={3}", n, n2, n + n2, n * n2);
            }
        }
    }
}
