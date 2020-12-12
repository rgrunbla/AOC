use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;
use std::cmp::Ordering;

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

    fn count(&self, index: (usize, usize)) -> (char, usize, usize, usize) {
        /*
        If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
        Otherwise, the seat's state does not change.
        */

        let mut adjacent_empty = 0;
        let mut adjacent_occupied = 0;
        let mut adjacent_floor = 0;

        for i in -1i32..2i32 {
            for j in -1i32..2i32 {
                if (i, j) == (0, 0) {
                    continue;
                }
                match (add(index.0, i), add(index.1, j)) {
                    (Some(i), Some(j)) => {
                        match self.get((i, j)) {
                            Some('L') => adjacent_empty += 1,
                            Some('#') => adjacent_occupied += 1,
                            Some('.') => adjacent_floor += 1,
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
        }

        (self.get(index).unwrap(), adjacent_empty, adjacent_occupied, adjacent_floor)
    }

    fn step(&mut self) -> bool {
        let mut changed: bool = false;
        let mut data_copy = self.data.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let index = (i, j);
                match self.count(index) {
                    ('L', _, 0, _) => {
                        data_copy[index.0][index.1] = '#';
                        changed = true;
                    },
                    ('#', _, i, _) => {
                        if i >= 4 {
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
            plan.print();
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

        println!("Occupied: {}", occupied);

        plan.print();
        
    } else {
        println!("Error");
    }
}