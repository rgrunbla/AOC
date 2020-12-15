#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "/home/remy/AOC/2020/15/input";
    let mut memory: HashMap<u64, u64> = HashMap::new();

    if let Ok(mut lines) = read_lines(filename) {
        let mut latest_spoken: u64 = 0;
        for line in lines {
            if let Ok(line) = line {
                let numbers: Vec<&str> = line.split(",").collect();
                for (i, number) in numbers.iter().enumerate() {
                    latest_spoken = number.parse::<u64>().unwrap();
                    memory.insert(latest_spoken, 1+i as u64);
                }
            }
        }
        let mut i: u64 = memory.len() as u64;
        latest_spoken = 0;
        i += 1;

        loop {
            if (i == 30000000) {
                println!("Latest Spoken: {}", latest_spoken);
                break;
            }
            match memory.entry(latest_spoken) {
                Occupied(entry) => {
                    let entry: u64 = *entry.into_mut();
                    memory.insert(latest_spoken, i);
                    latest_spoken = i-entry;
                },
                Vacant(entry) => {
                    memory.insert(latest_spoken, i);
                    latest_spoken = 0;
                }
            }
            i += 1;
        }
    } else {
        println!("Error");
        return;
    }
}