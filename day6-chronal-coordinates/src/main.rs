use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    let points = parse_points(file);
    part_one(&points);
    part_two(&points);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_points(file: File) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for l in BufReader::new(file).lines() {
        let s = l.expect("Could not read line");
        let tokens: Vec<&str> = s.split(',').collect();
        let x = tokens
            .get(0)
            .expect("Couldnt parse x")
            .trim()
            .parse::<i32>()
            .expect("Couldnt parse x as i32");
        let y = tokens
            .get(1)
            .expect("Couldnt parse y")
            .trim()
            .parse::<i32>()
            .expect("Couldnt parse y as i32");
        points.push(Point { x, y });
    }
    points
}

fn part_two(points: &[Point]) {
    let (x_min, x_max, y_min, y_max) = board_dim(points);
    let mut within = 0;
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let p = Point { x, y };
            let cl = distances(&p, points);
            if cl < 10000 {
                within += 1;
            }
        }
    }
    println!("{}", within);
}

fn part_one(points: &[Point]) {
    let (x_min, x_max, y_min, y_max) = board_dim(points);
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut blacklist: HashSet<String> = HashSet::new();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let p = Point { x, y };
            let cl = closest(&p, points);
            if let Some(point) = cl {
                let key = format!("{}{}", point.x, point.y);
                if y == y_min || x == x_min || y == y_max || x == x_max {
                    blacklist.insert(key.clone());
                    counts.remove(&key);
                }
                if !blacklist.contains(&key) {
                    match counts.get(&key) {
                        Some(c) => {
                            counts.insert(key, c + 1);
                        }
                        None => {
                            counts.insert(key, 1);
                        }
                    }
                }
            }
        }
        println!();
    }

    let mut res: Vec<(&String, &usize)> = counts.iter().collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(1);
    println!("{:?}", res);
}

fn distances(p: &Point, pts: &[Point]) -> i32 {
    let mut dist = 0;
    for pp in pts {
        dist += distance(p, &pp);
    }
    dist
}

fn closest(p: &Point, pts: &[Point]) -> Option<Point> {
    let mut closest = pts.get(0).expect("Couldnt get first point");
    let mut closest_dist = std::i32::MAX;
    let mut tie = false;
    for pp in pts {
        let d = distance(p, &pp);
        if d == closest_dist {
            tie = true;
        }
        if d < closest_dist {
            closest = pp;
            closest_dist = d;
            tie = false;
        }
    }
    if tie {
        print!(" . |");
        None
    } else {
        print!("{} {}|", closest.x, closest.y);
        Some(Point {
            x: closest.x,
            y: closest.y,
        })
    }
}

fn board_dim(points: &[Point]) -> (i32, i32, i32, i32) {
    let x_min = points
        .iter()
        .map(|p| p.x)
        .min()
        .expect("Failed to get x min");
    let x_max = points
        .iter()
        .map(|p| p.x)
        .max()
        .expect("Failed to get x max");
    let y_min = points
        .iter()
        .map(|p| p.y)
        .min()
        .expect("Failed to get y min");
    let y_max = points
        .iter()
        .map(|p| p.y)
        .max()
        .expect("Failed to get y max");
    (x_min - 1, x_max + 1, y_min - 1, y_max + 1)
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
