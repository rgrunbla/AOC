use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let filename = "/home/remy/AOC/2020/10/input";
    let mut adapters: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                adapters.push(line.parse::<i64>().unwrap());
            }
        }

        adapters.sort();
        let mut differences: HashMap<i64, i64> = HashMap::new();
        let mut iteradpater = adapters.iter();

        let mut cur = iteradpater.next();
        let mut next = iteradpater.next();

        loop {
            match (cur, next) {
                (None, None) => {
                    println!("Finished parsing.");
                    break;
                },
                (Some(cur), Some(next)) => {
                    let entry = differences.entry(next-cur).or_insert(1);
                    *entry += 1;
                },
                (_, _) => ()
            }
            cur = next;
            next = iteradpater.next();
        }

        println!("{:?}", differences);
        println!("{} * {} = {}", 1, 3, (&differences.get(&3)).unwrap()*(&differences.get(&1)).unwrap());
    } else {
        println!("Error");
    }
}