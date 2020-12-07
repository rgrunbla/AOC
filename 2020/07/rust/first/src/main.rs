use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_once(in_string: &str, car: char) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, car);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

type Pair<'a> = (i32, &'a str);

fn parse_line(line: &str) { //-> (&str, Pair) {
    // wavy gray bags contain 2 mirrored fuchsia bags, 2 mirrored chartreuse bags, 3 mirrored blue bags.
    let container = Regex::new(r"^([A-Za-z ]*) bags contain ").unwrap();
    let contained = Regex::new(r"(\d+) ([A-Za-z ]*) bags").unwrap();
    println!("{}", &container.captures(line).unwrap()[1]);
    for _match in container.captures_iter(line) {
        println!("{}, {}", &_match[1], &_match[2]);
    }
}

fn main() {
    let filename = "/home/remy/AOC/2020/07/input";

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                parse_line(&line);
                
            }
        }
    } else {
        println!("Error");
    }
}