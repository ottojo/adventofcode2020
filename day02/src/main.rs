use std::env;
use std::fs;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

fn is_valid(pol: Policy, pw: &str) -> bool {
    let count = pw.matches(pol.character).count();
    if count < pol.min || count > pol.max {
        return false;
    }
    return true;
}

fn is_valid_type2(pol: Policy, pw: &str) -> bool {
    if (pw.chars().nth(pol.min - 1).unwrap() == pol.character
        && pw.chars().nth(pol.max - 1).unwrap() != pol.character)
        || (pw.chars().nth(pol.min - 1).unwrap() != pol.character
            && pw.chars().nth(pol.max - 1).unwrap() == pol.character)
    {
        return true;
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let policyType = &args[2];

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
        let min: usize = line[0..dash].parse().expect("Minimum not found");
        let max: usize = line[dash + 1..colon - 2]
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
        if policyType == "1" {
            if is_valid(policy, pw) {
                println!("Valid (type 1)");
                count += 1;
            }
        } else if policyType == "2" {
            if is_valid_type2(policy, pw) {
                println!("Valid (type 2)");
                count += 1;
            }
        } else {
            panic!("Invalid policy type");
        }
    }

    println!("{} valid passwords", count);
}
