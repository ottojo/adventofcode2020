use std::convert::TryInto;
use std::env;
use std::fs;

#[derive(Debug)]
struct Policy {
    min: u64,
    max: u64,
    character: char,
}

fn is_valid(pol: Policy, pw: &str) -> bool {
    let count = pw.matches(pol.character).count();
    if count < pol.min.try_into().unwrap() || count > pol.max.try_into().unwrap() {
        return false;
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut count = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        println!("Checking {:?}", line);
        let dash = line.find("-").expect("Line contains no dash");
        let colon = line.find(":").expect("Line contains no colon");
        println!("Dash at index {:?}, colon at {:?}", dash, colon);
        let min: u64 = line[0..dash].parse().expect("Minimum not found");
        let max: u64 = line[dash + 1..colon - 2]
            .parse()
            .expect("Maximum not found");
        println!("Min: {:?}, Max: {:?}", min, max);
        let character: char = line.chars().nth(colon - 1).unwrap();
        println!("Character: {:?}", character);
        let policy = Policy {
            min: min,
            max: max,
            character: character,
        };
        println!("Policy: {:?}", policy);
        let pw = &line[colon + 2..];
        println!("PW: {:?}", pw);
        if is_valid(policy, pw) {
            count += 1;
        }
    }

    println!("{} valid passwords", count);
}
