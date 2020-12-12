use num_bigint::BigUint;
use num_traits::{Zero, One};
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
    adapters.push(0);

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

        /* Part 2 */
        adapters.push(*adapters.last().unwrap()+3);

        let mut counts: Vec<BigUint> = vec![Zero::zero(); adapters.len()];
        counts[0] = One::one();
        let diff = 3;
        for (i, item) in adapters.iter().enumerate() {
            for j in 0..diff {
                if i+j+1 < adapters.len() {
                    if (adapters[i+j+1] - item) <= diff as i64 {
                        counts[i+j+1] = counts[i+j+1].clone() + counts[i].clone();
                    } 
                }
            }
            if (i > 0) {
                counts[i-1] = Zero::zero();
            }
        }
        println!("{}", counts.iter().max().unwrap());
    } else {
        println!("Error");
    }
}
