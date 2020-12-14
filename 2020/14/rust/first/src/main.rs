#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_line(location: String, value: String) -> (usize, i64) {
    lazy_static! {
        static ref locationre: Regex = Regex::new(r"^mem\[(?P<location>\d+)\]").unwrap();
        static ref valuere: Regex = Regex::new(r"(?P<value>\d+)").unwrap();
    }
    let caps_location = locationre.captures(&location).unwrap();
    let caps_value = valuere.captures(&value).unwrap();
    return (caps_location.name("location").unwrap().as_str().parse::<usize>().unwrap(),
            caps_value.name("value").unwrap().as_str().parse::<i64>().unwrap());
}

fn main() {
    let filename = "/home/remy/AOC/2020/14/input";
    let mut original_bitmask: Vec<char> = Vec::new();
    let mut bitmask: Vec<char> = Vec::new();
    let mut memory: HashMap<usize, isize> = HashMap::new();

    if let Ok(mut lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let instruction: Vec<&str> = line.split("=").collect();
                let label = instruction[0].trim();
                let rval = instruction[1].trim();
                if label == "mask" {
                    original_bitmask = rval.chars().collect();
                    bitmask = original_bitmask.clone();    
                } else {
                    let (location, value) = parse_line(label.to_string(), rval.to_string());
                    let bits: String = format!("{:036b}", value).chars().rev().collect();
                    for (i, car) in bits.chars().enumerate() {
                        let idx = bitmask.len()-i-1;
                        if original_bitmask[idx] == 'X' {
                            bitmask[idx] = car;
                        }
                    }
                    let or: String =  original_bitmask.iter().collect(); 
                    let ne: String = bitmask.iter().collect();
                    let intval = isize::from_str_radix(&ne, 2).unwrap();
                    memory.insert(location, intval);
                }
            }
        }
        let mut res: isize = 0;
        for (k, v) in memory {
            res += v;
        }
        println!("{}", res);
    } else {
        println!("Error");
        return;
    }
}