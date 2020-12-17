use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use std::convert::TryInto;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "/home/remy/AOC/2020/17/input";

    const N: usize = 3;
    let mut space: HashSet<[isize; N]> = HashSet::new();

    if let Ok(lines) = read_lines(filename) {
        for (i, row) in lines.enumerate() {
            if let Ok(row) = row {
                for (j, item) in row.chars().enumerate() {
                    if item == '#' {
                        let mut position: [isize; N] = [0; N];
                        position[N-2] = i as isize;
                        position[N-1] = j as isize;
                        space.insert(position);
                    }
                }
            }
        }

        let mut shifts: HashSet<[isize; N]> = HashSet::new();
        let multi_prod = (0..N).map(|_| -1..2).multi_cartesian_product();
        for combination in multi_prod {
            shifts.insert(combination.try_into().unwrap_or_else(|v: Vec<isize>| panic!("Expected length {} and got {}", N, v.len())));
        }

        let zero: [isize; N] = [0; N];

        for _ in 0..6 {
            let occupied = space.clone();
            let mut birth: HashMap<[isize; N], usize> = HashMap::new();

            for item in &occupied {
                let mut active = 0;
                for shift in &shifts {
                    if *shift == zero {
                        continue;
                    }
                    let mut res: [isize; N] = *item;
                    for i in 0..N {
                        res[i] += shift[i];
                    }
                    
                    let entry = birth.entry(res).or_insert(0);
                    *entry += 1;

                    match occupied.get(&res) {
                        Some(_) => active += 1,
                        None => (),
                    }
                }
                if !(active == 2 || active == 3) {
                    space.remove(item);
                }
            }
            for (coordinates, neighbors) in birth.iter() {
                if *neighbors == 3 {
                    match occupied.get(coordinates) {
                        Some(_) => (),
                        None => {
                            space.insert(*coordinates);
                        }
                    }
                }
            }
        }
        println!("Active Cells: {}", space.len());
    } else {
        println!("Error");
    }
}