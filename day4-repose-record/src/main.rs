use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).expect("No file provided")).expect("Could not open file");
    part_one(file);
}

fn compute_guard_stats(file: File) -> HashMap<usize, HashMap<usize, usize>> {
    let entries = parse_entries(file);
    let mut guard_stats: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let guards: HashSet<usize> = entries.iter().map(|x| x.guard_id).collect();
    guards.iter().for_each(|x| {
        guard_stats.insert(*x, HashMap::new());
    });
    let mut sleep_start: Option<usize> = None;
    for e in entries {
        let mut mins = guard_stats
            .get(&e.guard_id)
            .expect("No mins entry for guard_id")
            .clone();
        let this_min = e
            .time
            .time()
            .format("%M")
            .to_string()
            .parse::<usize>()
            .expect("could not parse second");

        match sleep_start {
            Some(s) => {
                if e.awake {
                    for i in s..this_min {
                        match mins.get(&i) {
                            Some(v) => {
                                mins.insert(i, v + 1);
                            }
                            None => {
                                mins.insert(i, 1);
                            }
                        }
                    }
                }
            }
            None => {}
        }
        sleep_start = Some(this_min);
        guard_stats.insert(e.guard_id, mins);
    }
    guard_stats
}
fn part_one(file: File) {
    let guard_stats = compute_guard_stats(file);
    let mut totals: Vec<(&usize, usize)> = guard_stats
        .iter()
        .map(|(k, v)| (k, v.iter().map(|(_, vv)| *vv).fold(0, |acc, x| acc + x)))
        .collect();
    totals.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    let (sleepiest_guard, _) = totals.first().expect("Couldnt get value");

    let mut stats: Vec<(&usize, &usize)> = guard_stats
        .get(sleepiest_guard)
        .expect("Couldnt find guard stats")
        .iter()
        .map(|(k, v)| (k, v))
        .collect();
    stats.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));

    let (sleepiest_min, _) = stats.first().expect("Couldnt get value");

    println!("Guard is {}, Minute is {}", sleepiest_guard, sleepiest_min);
    println!("Result is {}", **sleepiest_guard * **sleepiest_min);
}

fn parse_entries(file: File) -> Vec<LogEntry> {
    let mut entries: Vec<LogEntry> = Vec::new();
    for l in BufReader::new(file).lines() {
        let s = l.expect("Could not parse line");
        let tokens: Vec<&str> = s.split(&['[', ']'][..]).collect();
        let time = NaiveDateTime::parse_from_str(
            tokens.get(1).expect("Date element wasn't present"),
            "%Y-%m-%d %H:%M",
        )
        .expect("Could not unwrap datetime");

        let rem = tokens.get(2).expect("Element wasn't present");
        let mut awake = true;
        if rem.contains("falls asleep") {
            awake = false;
        }
        let mut guard_id = 0;
        if rem.contains("begins shift") {
            let tokens: Vec<&str> = rem.split(&['#', ' '][..]).collect();
            guard_id = tokens
                .get(3)
                .expect("Guard id not present")
                .parse::<usize>()
                .expect("Could not parse guard id");
        }
        let entry = LogEntry {
            time,
            guard_id,
            awake,
        };
        entries.push(entry);
    }
    entries.sort_by(|a, b| Ord::cmp(&a.time, &b.time));
    let mut last_id = 0;
    for mut e in &mut entries {
        if e.guard_id == 0 {
            e.guard_id = last_id;
        } else {
            last_id = e.guard_id;
        }
    }
    entries
}

#[derive(Debug)]
struct LogEntry {
    time: NaiveDateTime,
    guard_id: usize,
    awake: bool,
}
