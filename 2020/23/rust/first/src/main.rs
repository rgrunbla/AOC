use nohash_hasher::IntMap;
use std::fmt;
use std::fs;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() -> Result<(), Box<std::error::Error>>{
    let filename = "/home/remy/AOC/2020/23/input";
    let data = fs::read_to_string(filename).unwrap();
    let mut cups: IntMap<i64, i64> = IntMap::default();
    let chars: Vec<char> = data.trim().chars().collect();
    for i in 0..chars.len() {
        if i == 0 {
            continue;
        }
        cups.insert(chars[i-1].to_digit(10).unwrap() as i64, chars[i].to_digit(10).unwrap() as i64);
    }


    let mut maxi = 0;
    for (key, _) in &cups {
        if *key > maxi {
            maxi = *key;
        }
    }

    let mut last = chars[chars.len()-1].to_digit(10).unwrap() as i64;
    for i in maxi+1..=1000000 {
        cups.insert(last, i);
        last = i;
    }
    
    cups.insert(last, chars[0].to_digit(10).unwrap() as i64);

    let mut cur = chars[0].to_digit(10).unwrap() as i64;

    let mut maxi = 0;
    for (key, _) in &cups {
        if *key > maxi {
            maxi = *key;
        }
    }

    for i in 0..10000000 {
        /*
        let mut icur = cur;
        for _ in 0..maxi {
            print!("{} ", cups.get(&icur).unwrap());
            icur = *cups.get(&icur).unwrap();
        }
        println!("");
        println!("{:?}", cups);*/
        let mut selected : [i64; 3] = [0; 3];
        selected[0] = *cups.get(&cur).unwrap();
        selected[1] = *cups.get(&selected[0]).unwrap();
        selected[2] = *cups.get(&selected[1]).unwrap();
        cups.insert(cur, *cups.get(&selected[2]).unwrap());
        //println!("pick up: {:?}", selected);
        let mut destination = &cur - 1;
        while destination == selected[0] || destination == selected[1] || destination == selected[2] {
            destination -= 1;
            if destination <= 0 {
                destination = maxi;
            }
        }
        if destination <= 0 {
            destination = maxi;
        }
        //println!("destination: {}", destination);
        let next = cups.get(&destination).unwrap();
        cups.insert(selected[2], *next);
        cups.insert(destination, selected[0]);
        cur = *cups.get(&cur).unwrap();
    }

    println!("{}", cups.get(&1).unwrap());
    println!("{}", cups.get(cups.get(&1).unwrap()).unwrap());

    Ok(())
}
