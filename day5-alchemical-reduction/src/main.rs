use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    let input: Vec<char> = BufReader::new(file)
        .lines()
        .nth(0)
        .expect("Could not parse line")
        .expect("Could not parse line")
        .chars()
        .collect();

    println!("Part 1 result: {}", process(input.clone()));
    let mut min = std::usize::MAX;
    let ascii_iter = (0..26).map(|x| (x + b'a') as char);

    for c in ascii_iter {
        let test: Vec<char> = input
            .clone()
            .into_iter()
            .filter(|x| x.to_ascii_lowercase() != c)
            .collect();
        min = std::cmp::min(min, process(test));
    }
    println!("Part 2 result: {}", min);
}

fn process(input: Vec<char>) -> usize {
    let mut foo = input.clone();
    let mut l = 0;
    let mut r = 1;
    while r < foo.len() {
        if react(foo[l], foo[r]) {
            foo = [&foo[..l], &foo[r + 1..foo.len()]].concat();
            l = 0;
            r = 1;
        } else {
            l = l + 1;
            r = r + 1;
        }
    }
    foo.len()
}

fn react(l: char, r: char) -> bool {
    (l != r) && (l.to_ascii_uppercase() == r.to_ascii_uppercase())
}
