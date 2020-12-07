#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

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

fn parse_line(line: String, graph: &mut HashMap<String, Vec<(String, i32)>>) {
    lazy_static! {
        static ref container: Regex = Regex::new(r"^([A-Za-z ]*) bags contain ").unwrap();
        static ref contained: Regex = Regex::new(r"(\d+) ([A-Za-z ]*) bag").unwrap();
    }

    let container_capture = &container.captures(line.as_str()).unwrap()[1];
    for _match in contained.captures_iter(line.as_str()) {
        graph.entry(container_capture.to_string())
        .or_insert(vec![])
        .push((_match[2].to_string(),  _match[1].to_string().parse::<i32>().unwrap()));
    }
}

fn dfs(key: String, graph: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    match graph.get(&key) {
        Some(item) => {
            let mut acc: i32 = 0;
            for color in item {
                acc += color.1 * (1 + dfs(color.0.clone(), &graph));
            }
            acc
        },
        None => {
            0
        }
    }
}

fn main() {
    let mut graph = HashMap::new();

    let filename = "/home/remy/AOC/2020/07/input";

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                parse_line(line, &mut graph);
            }
        }
        println!("Fin du parsing");
        println!("{}", dfs(String::from("shiny gold"), &graph));
    } else {
        println!("Error");
    }
}