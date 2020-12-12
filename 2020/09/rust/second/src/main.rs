use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;

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
                let found_number = numbers[idx];
                println!("The number {} cannot be created with the previous {} numbers.", found_number, size);
                
                let mut sub: VecDeque<i64> = VecDeque::new();
                let mut sum: i64 = 0;
                let mut veciter = numbers.iter();
                loop {
                    println!("Sub: {:?}", sub);
                    println!("Sum: {}", sum);

                    if sum == found_number {
                        println!("!!! Sum: {}", sum);
                        sub.make_contiguous().sort();
                        println!("Min: {}, Max: {}, Sum: {}", sub.front().unwrap(), sub.back().unwrap(), sub[0]+sub.back().unwrap());
                        return;
                    } else if sum > found_number {
                        sum -= sub[0];
                        sub.pop_front();
                    } else {
                        let item = veciter.next();
                        match item {
                            None => { println!("Error, subarray not found."); return;}
                            Some(item) => {
                                sub.push_back(*item);
                                sum += item;
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", numbers);
    } else {
        println!("Error");
    }
}