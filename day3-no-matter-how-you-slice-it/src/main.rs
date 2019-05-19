use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    //part_one(file);
    part_two(file);
}

fn part_two(file: File) {
    let claims: Vec<Claim> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not read line"))
        .map(|l| parse_claim(l))
        .collect();

    for (i, c) in claims.iter().enumerate() {
        if i < claims.len() {
            let mut overlapped = false;
            for o in &claims {
                if overlap(c, o) {
                    overlapped = true;
                    break;
                }
            }
            if overlapped == false {
                println!("ID {} didn't overlap", c.id);
            }
        }
    }
}

fn overlap(a: &Claim, b: &Claim) -> bool {
    // If it's the same claim, don't count as overlap
    if a.id == b.id {
        return false;
    }
    // If either is to the right of the other
    if a.x_margin + 1 > b.x_margin + b.x_size || b.x_margin + 1 > a.x_margin + a.x_size {
        return false;
    }
    // If either is below the other
    if a.y_margin + 1 > b.y_margin + b.y_size || b.y_margin + 1 > a.y_margin + a.y_size {
        return false;
    }
    true
}

fn part_one(file: File) {
    let claims: Vec<Claim> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not read line"))
        .map(|l| parse_claim(l))
        .collect();

    let mut multi_claimed = 0;
    for y in 1..1001 {
        for x in 1..1001 {
            if is_multi_claimed((x, y), &claims) {
                multi_claimed = multi_claimed + 1;
            }
        }
    }
    println!("{}", multi_claimed);
}

fn is_multi_claimed((x, y): (usize, usize), claims: &Vec<Claim>) -> bool {
    let mut claim_count = 0;
    for c in claims {
        if within((x, y), c) {
            claim_count = claim_count + 1;
            if claim_count > 1 {
                return true;
            }
        }
    }
    false
}

fn within((x, y): (usize, usize), claim: &Claim) -> bool {
    if x > claim.x_margin
        && x <= claim.x_margin + claim.x_size
        && y > claim.y_margin
        && y <= claim.y_margin + claim.y_size
    {
        return true;
    }
    false
}

fn parse_claim(s: String) -> Claim {
    let parsed: Vec<&str> = s.split(&['#', '@', ',', ':', 'x'][..]).collect();
    let id = parsed
        .get(1)
        .expect("Missing element")
        .trim()
        .parse::<usize>()
        .expect("Could not parse usize from element");
    let x_margin = parsed
        .get(2)
        .expect("Missing element")
        .trim()
        .parse::<usize>()
        .expect("Could not parse usize from element");
    let y_margin = parsed
        .get(3)
        .expect("Missing element")
        .trim()
        .parse::<usize>()
        .expect("Could not parse usize from element");
    let x_size = parsed
        .get(4)
        .expect("Missing element")
        .trim()
        .parse::<usize>()
        .expect("Could not parse usize from element");
    let y_size = parsed
        .get(5)
        .expect("Missing element")
        .trim()
        .parse::<usize>()
        .expect("Could not parse usize from element");

    Claim {
        id,
        x_margin,
        y_margin,
        x_size,
        y_size,
    }
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x_margin: usize,
    y_margin: usize,
    x_size: usize,
    y_size: usize,
}
