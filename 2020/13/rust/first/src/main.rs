use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;
use std::cmp::Ordering;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "/home/remy/AOC/2020/13/input";
    let mut arrival: i64 = 0;
    let mut bus_frequencies: Vec<i64> = Vec::new();

    if let Ok(mut lines) = read_lines(filename) {
        arrival = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
        for i in lines.next().unwrap().unwrap().split(",") {
            match i.parse::<i64>() {
                Ok(i) => bus_frequencies.push(i),
                _ => bus_frequencies.push(-1)
            }
        }
    } else {
        println!("Error");
        return;
    }
    let mut waiting_time: i64 = i64::MAX;
    let mut best_bus: i64 = -1;

    for item in &bus_frequencies {
        if *item == -1 {
            continue;
        }
        let next_bus =  ((arrival / item) + 1) * item;
        if  next_bus - arrival < waiting_time {
            waiting_time = next_bus - arrival;
            best_bus = *item;
        }
    }
    println!("{} * {} = {}", waiting_time, best_bus, waiting_time * best_bus);

    let mut idx_first = 0;
    let mut idx_second = 1;
    let mut partial_solution: i64 = 0;
    let mut mult_factor: i64 = bus_frequencies[0];

    loop {
        match (bus_frequencies.get(idx_first), bus_frequencies.get(idx_second)) {
            (Some(i), None) => break,
            (Some(i), Some(-1)) => idx_second += 1,
            (Some(i), Some(j)) => {
                while partial_solution.rem_euclid(*j) != (-(idx_second as i64)).rem_euclid(*j) {
                    partial_solution += mult_factor;
                }
                idx_first = idx_second;
                idx_second += 1;
                mult_factor *= (*j as i64);
            }
            _ => panic!("Something is fishy with the input")
                
        }
    }
    println!("Solution: {}", partial_solution);
}