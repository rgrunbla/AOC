use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let filename = "/home/remy/AOC/2020/09/input";
    let mut numbers: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                numbers.push(line.parse::<i64>().unwrap());
            }
        }

        let mut window: HashSet<i64> = HashSet::new();
        let size = 25;
        for i in 0..size {
            window.insert(numbers[i]);
        }
        for idx in size..numbers.len() {
            let mut found = false;
            for item in &window {
                if window.contains(&(numbers[idx]-item)) {
                    println!("The number {} can be created with {} and {}", numbers[idx], item, numbers[idx]-item);
                    window.insert(numbers[idx]);
                    window.remove(&numbers[idx-size]);
                    found = true;
                    break;
                }
            }
            if !found {
                println!("The number {} cannot be created with the previous {} numbers.", numbers[idx], size);
                return;
            }
        }
        /*

        */
        println!("{:?}", numbers);
    } else {
        println!("Error");
    }
}