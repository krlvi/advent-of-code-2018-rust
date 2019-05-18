use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    part_one(&file);
    part_two(file);
}

fn part_one(file: &File) {
    let mut freq = 0;
    for l in BufReader::new(file).lines() {
        let val = l
            .expect("Could not read line")
            .parse::<i32>()
            .expect("Could not parse int from line");
        freq = freq + val;
    }
    println!("{}", freq);
}

fn part_two(file: File) {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut freq = 0;
    let foo: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("couldnt read line")
                .parse::<i32>()
                .expect("couldnt parse line")
        })
        .collect();

    for val in foo.iter().cycle() {
        freq = freq + val;

        if seen.contains(&freq) {
            println!("{}", freq);
            break;
        } else {
            seen.insert(freq);
        }
    }
}
