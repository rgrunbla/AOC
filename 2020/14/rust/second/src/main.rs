#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_line(location: String, value: String) -> (i64, i64) {
    lazy_static! {
        static ref locationre: Regex = Regex::new(r"^mem\[(?P<location>\d+)\]").unwrap();
        static ref valuere: Regex = Regex::new(r"(?P<value>\d+)").unwrap();
    }
    let caps_location = locationre.captures(&location).unwrap();
    let caps_value = valuere.captures(&value).unwrap();
    return (caps_location.name("location").unwrap().as_str().parse::<i64>().unwrap(),
            caps_value.name("value").unwrap().as_str().parse::<i64>().unwrap());
}

fn try_all(address: String, addresses: &mut HashSet<i64>) {
    match address.find('X') {
        Some(i) => {
            let mut ns: Vec<char> = address.chars().collect();
            ns[i] = '0';
            try_all(ns.iter().collect(), addresses);
            ns[i] = '1';
            try_all(ns.iter().collect(), addresses);
        },
        None => {
            let intval = i64::from_str_radix(&address, 2).unwrap();
            addresses.insert(intval);
        }
    }
}

fn main() {
    let filename = "/home/remy/AOC/2020/14/input";
    let mut original_bitmask: Vec<char> = Vec::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();

    if let Ok(mut lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let instruction: Vec<&str> = line.split("=").collect();
                let label = instruction[0].trim();
                let rval = instruction[1].trim();
                if label == "mask" {
                    original_bitmask = rval.chars().collect();
                } else {
                    let (location, value) = parse_line(label.to_string(), rval.to_string());
                    let mut bits: String = format!("{:036b}", location).chars().rev().collect();
                    let mut bitmask: Vec<char> = bits.chars().collect::<Vec<char>>().clone();    

                    for (i, car) in bits.chars().enumerate() {
                        let idx = bitmask.len()-i-1;
                        match original_bitmask[idx] {
                        'X' => bitmask[idx] = 'X',
                        '1' => bitmask[idx] = '1',
                        '0' => bitmask[idx] = car,
                        _ => ()
                        }
                    }

                    let or: String =  original_bitmask.iter().collect(); 
                    let ne: String = bitmask.iter().collect();
                    bits = format!("{:036b}", location).chars().collect();

                    let mut addresses: HashSet<i64> = HashSet::new();
                    try_all(ne, &mut addresses);
                    for item in addresses {
                        memory.insert(item, value);
                    }
                }
            }
        }
        let mut res: i64 = 0;
        for (k, v) in memory {
            res += v;
        }
        println!("{}", res);
    } else {
        println!("Error");
        return;
    }
}
