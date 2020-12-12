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

enum Actions {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(f32),
    R(f32),
    F(f32)
}

struct Position {
    x: i32,
    y: i32,
    facing: f32
}

fn main() {
    let filename = "/home/remy/AOC/2020/12/input";
    let mut actions: Vec<Actions> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                match &line[0..1] {
                    "N" => actions.push(Actions::N(line[1..].parse::<i32>().unwrap())),
                    "S" => actions.push(Actions::S(line[1..].parse::<i32>().unwrap())),
                    "E" => actions.push(Actions::E(line[1..].parse::<i32>().unwrap())),
                    "W" => actions.push(Actions::W(line[1..].parse::<i32>().unwrap())),
                    "L" => actions.push(Actions::L(line[1..].parse::<f32>().unwrap().to_radians())),
                    "R" => actions.push(Actions::R(line[1..].parse::<f32>().unwrap().to_radians())),
                    "F" => actions.push(Actions::F(line[1..].parse::<f32>().unwrap())),
                    _ => panic!("Unknown data")
                }
            }
        }
        
    let mut position: Position = Position{x : 0, y : 0, facing : 0.0};

    for action in actions {
        println!("({}, {}) facing {}", position.x, position.y, position.facing);
        match action {
            Actions::N(i) => position.y += i,
            Actions::S(i) => position.y -= i,
            Actions::E(i) => position.x += i,
            Actions::W(i) => position.x -= i,
            Actions::L(a) => position.facing += a,
            Actions::R(a) => position.facing -= a,
            Actions::F(i) => {
                let x = position.facing.cos() * i;
                let y = position.facing.sin() * i;
                position.x += (x as i32);
                position.y += (y as i32);
            }
        }
    }
        
    println!("End position: {}, {}, {}", position.x, position.y, position.facing);
    
    } else {
        println!("Error");
    }
}