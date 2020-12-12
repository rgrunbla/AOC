use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Actions {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(f32),
    R(f32),
    F(i32)
}

struct Position {
    x: i32,
    y: i32
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
                    "F" => actions.push(Actions::F(line[1..].parse::<i32>().unwrap())),
                    _ => panic!("Unknown data")
                }
            }
        }
        
        let mut waypoint: Position = Position{x: 10, y : 1};
        let mut position: Position = Position{x : 0, y : 0};

        for action in actions {
            match action {
                Actions::N(i) => waypoint.y += i,
                Actions::S(i) => waypoint.y -= i,
                Actions::E(i) => waypoint.x += i,
                Actions::W(i) => waypoint.x -= i,
                Actions::L(a) => {
                    let newx = (waypoint.x as f32) * a.cos() - (waypoint.y as f32) * a.sin();
                    let newy =(waypoint.x as f32) * a.sin() + (waypoint.y as f32) * a.cos();
                    waypoint.x = newx.round() as i32;
                    waypoint.y = newy.round() as i32;
                },
                Actions::R(a) => {
                    let newx = (waypoint.x as f32) * (-a).cos() - (waypoint.y as f32) * (-a).sin();
                    let newy =(waypoint.x as f32) * (-a).sin() + (waypoint.y as f32) * (-a).cos();
                    waypoint.x = newx.round() as i32;
                    waypoint.y = newy.round() as i32;
                },
                Actions::F(i) => {
                    for _ in 0..i {
                        position.x += waypoint.x;
                        position.y += waypoint.y;

                    }
                }
            }
        }
        println!("Manhattan: {}", position.x.abs() + position.y.abs());
    } else {
        println!("Error");
    }
}
