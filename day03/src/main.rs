use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum MapSquare {
    Tree,
    Open,
}

type MapLine = Vec<MapSquare>;
type Map = Vec<MapLine>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // Map filename
    let challenge = &args[2]; // 1 or 2

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut map = Map::new();

    for line in lines {
        let mut map_line = MapLine::new();
        for character in line.chars() {
            if character == '#' {
                map_line.push(MapSquare::Tree);
            } else if character == '.' {
                map_line.push(MapSquare::Open);
            }
        }
        if !map_line.is_empty() {
            map.push(map_line);
        }
    }

    println!("{:?}", map);

    if challenge == "1" {
        println!("{} trees found", count_trees(&map, 3, 1));
    } else {
        let mut product = 1;
        product *= test(&map, 1, 1);
        product *= test(&map, 3, 1);
        product *= test(&map, 5, 1);
        product *= test(&map, 7, 1);
        product *= test(&map, 1, 2);
        println!("Product of those numbers: {}", product);
    }
}

fn test(map: &Map, right: usize, down: usize) -> usize {
    let trees = count_trees(&map, right, down);
    println!("Right {}, Down {}: {} trees found", right, down, trees);
    return trees;
}

fn count_trees(map: &Map, right: usize, down: usize) -> usize {
    let mut x: usize = 0;
    let mut trees = 0;
    for line in map.iter().step_by(down) {
        if line[x % line.len()] == MapSquare::Tree {
            trees += 1;
        }
        x += right;
    }
    return trees;
}
