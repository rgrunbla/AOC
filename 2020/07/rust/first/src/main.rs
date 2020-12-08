use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn parse_line<'a>(line: &'a str, graph: &mut HashMap<String, Vec<String>>) {
    // light chartreuse bags contain 1 mirrored yellow bag, 2 vibrant violet bags.
    let container = Regex::new(r"^([A-Za-z ]*) bags contain ").unwrap();
    let container =  &container.captures(line).unwrap()[1];
    let contained = Regex::new(r"(\d+) ([A-Za-z ]*) bag").unwrap();

    for _match in contained.captures_iter(line) {
        graph.entry(_match[2].to_string()).or_insert(vec![container.to_string()]).push(container.to_string());
    }
}

fn main() {
    let mut graph = HashMap::new();

    let filename = "/home/remy/AOC/2020/07/input";

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                parse_line(&line, &mut graph);   
            }
        }
        let mut set = HashSet::new();
        let mut size = 0;
        set.insert(String::from("shiny gold"));
        loop {
            if set.len() == size {
                break;
            }
            size = set.len();
            for key in set.clone() {
                match graph.get(&key) {
                    Some(item) => for color in item { set.insert(color.to_string()); },
                    None => println!("{} has no parent", key)
                }
            }
        }
        println!("Set has size {}, result: {}", set.len(), set.len()-1);
    } else {
        println!("Error");
    }
}
