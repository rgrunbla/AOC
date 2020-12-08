use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_once(in_string: &str, car: char) -> (String, String) {
    let mut splitter = in_string.splitn(2, car);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first.to_string(), second.to_string())
}
fn main() {

    let filename = "/home/remy/AOC/2020/08/input";
    let mut instructions: Vec<(String, i32)> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let (first, second) = split_once(&line[..], ' ');
                instructions.push((first, second.parse::<i32>().unwrap()));
            }
        }
        println!("Instructions: {:?}", instructions);
        let mut visited: HashSet<usize> = HashSet::new();
        let mut acc = 0;
        let mut idx: usize = 0;
        loop {
            if visited.contains(&idx) {
                println!("Loop detected !");
                return;
            } else {
                visited.insert(idx);
            }

            let (instruction, argument) = &instructions[idx];
            match (instruction.as_str(), argument) {
                ("nop", arg) => {
                    println!("{}, NOP {}", idx, arg);
                    idx += 1;
                },
                ("acc", arg) => {
                    println!("{}, ACC {}", idx, arg);
                    acc += *arg;
                    idx += 1;
                },
                ("jmp", arg) => {
                    println!("{}, JMP {} [{}]", idx, arg, acc);
                    if *arg > 0 {
                        idx += *arg as usize;
                    } else {
                        idx -= (-*arg) as usize;
                    }
                },
                _ => {
                    println!("Invalid instruction");
                    return;
                }
            }
        }
    } else {
        println!("Error");
    }
}