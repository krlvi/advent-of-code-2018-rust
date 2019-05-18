use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    //part_one(file);
    part_two(file);
}

fn part_one(file: File) {
    let mut twos = 0;
    let mut threes = 0;
    for l in BufReader::new(file).lines() {
        let e: Vec<char> = l.expect("Could not parse line").chars().collect();
        count(&e, &mut twos, &mut threes);
    }
    println!("{}", twos * threes);
}

fn count(chars: &Vec<char>, twos: &mut i32, threes: &mut i32) {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in chars {
        match freq.get(&c) {
            Some(v) => {
                freq.insert(*c, v + 1);
            }
            None => {
                freq.insert(*c, 1);
            }
        }
    }
    if freq.values().any(|v| *v == 2) {
        *twos = *twos + 1;
    }
    if freq.values().any(|v| *v == 3) {
        *threes = *threes + 1;
    }
}

fn is_pair(a: &Vec<char>, b: &Vec<char>) {
    let mut dif = 0;
    for (ai, bi) in a.iter().zip(b.iter()) {
        if ai != bi {
            dif = dif + 1;
        }
    }
    if dif == 1 {
        println!("{:?}", a);
        println!("{:?}", b);
    }
}

fn part_two(file: File) {
    let mut input: Vec<Vec<char>> = Vec::new();
    for l in BufReader::new(file).lines() {
        input.push(l.expect("Could not parse line").chars().collect());
    }
    for i in input.iter() {
        for j in input.iter() {
            is_pair(i, j);
        }
    }
}
