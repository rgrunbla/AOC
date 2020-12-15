#![feature(box_syntax, box_patterns)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "/home/remy/AOC/2020/15/input";
    let mut memory = box [u32::MAX; 30000000];
    
    if let Ok(lines) = read_lines(filename) {
        let mut latest_spoken: u32;
        let mut i: u32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let numbers: Vec<&str> = line.split(",").collect();
                for (idx, number) in numbers.iter().enumerate() {
                    i += 1;
                    memory[number.parse::<u32>().unwrap() as usize] = (1+idx) as u32;
                }
            }
        }
        latest_spoken = 0;
        i += 1;
        for i in i..30000000 {
            unsafe {
                let tmp = memory.get_unchecked_mut(latest_spoken as usize);
                match tmp {
                    &mut u32::MAX => {
                        *tmp = i;
                        latest_spoken = 0;
                    },
                    &mut entry => {
                        memory[latest_spoken as usize] = i;
                        latest_spoken = i-entry;
                    }
                }
            }
        }
        println!("{}", latest_spoken);
    } else {
        println!("Error");
        return;
    }
}