#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;
use std::cmp::Ordering;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Constraint {
    name: String,
    intervals: Vec<(u32, u32)>
}

fn main() {
    let filename = "/home/remy/AOC/2020/16/input";
    let data = fs::read_to_string(filename).unwrap();
    let data: Vec<&str> = data.split("\n\n").collect();
    
    let constraints_str = data[0].to_string();
    let my_ticket_str = data[1].to_string();
    let nearby_tickets_str = data[2].to_string();

    let mut constraints: Vec<Constraint> = Vec::new();

    /* Ugly Constraint Parsing */
    for constraint in constraints_str.lines() {
        let splitted: Vec<&str> = constraint.split(':').collect();
        let label = splitted[0].to_string();
        let mut intervals: Vec<(u32, u32)> = Vec::new();
        for constraint in splitted[1].split("or") {
            let constraint: Vec<&str> = constraint.split("-").collect();
            intervals.push(
                (constraint[0].trim().parse::<u32>().unwrap(),
                 constraint[1].trim().parse::<u32>().unwrap())
            );
        }
        constraints.push(Constraint{name: label, intervals});
    }

    /* Ugly Ticket Parsing */
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut lines = my_ticket_str.lines();
    lines.next();
    let numbers = lines.next().unwrap().to_string();
    for number in numbers.split(",") {
        my_ticket.push(number.trim().parse::<u32>().unwrap());
    }

    /* Ugly Nearby Ticket Parsing */
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
    let mut lines = nearby_tickets_str.lines();
    lines.next();
    for line in lines {
        let mut nearby_ticket: Vec<u32> = Vec::new();
        let numbers = line.to_string();
        for number in numbers.split(",") {
            nearby_ticket.push(number.trim().parse::<u32>().unwrap());
        }
        nearby_tickets.push(nearby_ticket);
    }


    let mut valid_count = 0;
    let mut nearby_valid_tickets: Vec<Vec<u32>> = Vec::new();

    for nearby_ticket in &nearby_tickets {
        let mut ticket_is_valid = true;
        for number in nearby_ticket {
            let mut is_valid: bool = false;
            for constraint in &constraints {
                for (min, max) in &constraint.intervals {
                    if min <= number && number <= max {
                        is_valid = true;
                        break;
                    }
                }
                if is_valid {
                    break;
                }
            }
            if !is_valid {
                valid_count += number;
                ticket_is_valid = false;
            }
        }
        if ticket_is_valid {
            nearby_valid_tickets.push(nearby_ticket.clone());
        }
    }
    println!("Result 1: {}", valid_count);

    let mut attribution: Vec<HashSet<String>> = Vec::new();
    let mut hs: HashSet<String> = HashSet::new();
    for constraint in &constraints {
        hs.insert(constraint.name.clone());
    }

    for (pos, number) in my_ticket.iter().enumerate() {
        attribution.push(hs.clone());
    }

    for nearby_ticket in &nearby_valid_tickets {
        for (pos, number) in nearby_ticket.iter().enumerate() {
            for constraint in &constraints {
                let mut is_valid: bool = false;
                for (min, max) in &constraint.intervals {
                    if (min <= number && number <= max) {
                        is_valid = true;
                        break;
                    }
                }
                if !is_valid {
                    attribution[pos].remove(&constraint.name);
                }
            }
        }
    }

    let mut attributeds: HashSet<String> = HashSet::new();
    let mut attributions: HashMap<String, usize> = HashMap::new();
    loop {
        let mut finished: bool = true;
        for (i, possibility) in &mut attribution.iter().enumerate() {
            match possibility.len() {
                0 => (),
                1 => {
                    finished = false;
                    let values: Vec<&String> = possibility.iter().collect();
                    attributeds.insert(values[0].to_string());
                    attributions.insert(values[0].to_string(), i);
                },
                _ => {
                    finished = false;
                }
            }
        }
        if finished {
            break;
        }
        for attributed in &attributeds {
            for possibility in &mut attribution {
                possibility.remove(attributed);
            }
        }
    }

    let mut result: i64 = 1;
    for (key, value) in attributions {
        if key.starts_with("departure") {
            result *= my_ticket[value] as i64;
        } else {
        }
    }
    println!("Result 2: {}", result);
}