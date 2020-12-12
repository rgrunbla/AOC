use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn add(size: usize, shift: i32) -> Option<usize> {
    let ret: i32 = shift + (size as i32);
    if ret < 0 {
        return None;
    }
    return Some(ret as usize);
}

struct Plan {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize
}

impl Default for Plan {
    fn default() -> Self {
        let vec: Vec<Vec<char>> = Vec::new();
        Plan {
            data: vec,
            height: 0,
            width: 0
        }
    }
}

impl Plan {
    fn new() -> Self {
        Default::default()
    }

    fn push_row(&mut self, line: String) {
        let row: Vec<char> = line.chars().collect();
        let rowlen: usize = row.len();
        self.data.push(row);
        self.height += 1;
        self.width = max(self.width, rowlen);
    }

    fn print(&self) {
        println!("");
        for row in &self.data {
            let row: String = row.into_iter().collect();
            println!("{}", row);
        }
        println!("");
    }

    fn get(&self, index: (usize, usize)) -> Option<char> {
        if let Some(v) = self.data.get(index.0) {
            if let Some(item) = v.get(index.1) {
                return Some(*item)
            }
        }
        return None
    }

    fn count(&self, index: (usize, usize)) -> (char, usize, usize) {
        let mut adjacent_empty = 0;
        let mut adjacent_occupied = 0;

        for i_shift in -1..2 {
            for j_shift in -1..2 {
                /* Skip self */
                if (i_shift, j_shift) == (0, 0) {
                    continue;
                }
                /* Skip out-of-bound index */
                if let (Some(i), Some(j)) = (add(index.0, i_shift), add(index.1, j_shift)) {
                    match self.get((i, j)) {
                        Some('L') => adjacent_empty += 1,
                        Some('#') => adjacent_occupied += 1,
                        Some('.') => {
                            let mut first_i = i;
                            let mut first_j = j;
                            while let (Some(i), Some(j)) = (add(first_i, i_shift), add(first_j, j_shift)) {
                                match self.get((i, j)) {
                                    Some('.') => {
                                        first_i = i;
                                        first_j = j;
                                        continue;
                                    },
                                    Some('L') => adjacent_empty += 1,
                                    Some('#') => adjacent_occupied += 1,
                                    _ => ()
                                }
                                break;
                            }
                        }
                        _ => ()
                    }
                }
            }
        }
        (self.get(index).unwrap(), adjacent_empty, adjacent_occupied)
    }

    fn step(&mut self) -> bool {
        let mut changed: bool = false;
        let mut data_copy = self.data.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let index = (i, j);
                match self.count(index) {
                    ('L', _, 0) => {
                        data_copy[index.0][index.1] = '#';
                        changed = true;
                    },
                    ('#', _, i) => {
                        if i >= 5 {
                            data_copy[index.0][index.1] = 'L';
                            changed = true;
                        }
                    },
                    _ => ()

                }
            }
        }
        self.data = data_copy;
        return changed;
    }
}

fn main() {
    let filename = "/home/remy/AOC/2020/11/input";
    let mut plan: Plan = Plan::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                plan.push_row(line);
            }
        }
        plan.print();

        while plan.step() {

        }    
        
        let mut occupied: i32 = 0;
        for i in 0..plan.height {
            for j in 0..plan.width {
                match plan.get((i, j)) {
                    Some('#') => occupied += 1,
                    _ => ()
                }
            }
        }

        plan.print();
        println!("Occupied: {}", occupied);
        
    } else {
        println!("Error");
    }
}