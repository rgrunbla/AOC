use std::fmt;
use std::fs;
extern crate nom;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, space0},
    combinator::map_res,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

use std::str::FromStr;

fn direction(i: &str) -> IResult<&str, &str> { alt((tag("e"), tag("se"), tag("sw"), tag("w"), tag("nw"), tag("ne")))(i) }
fn position(i: &str) -> IResult<&str, Vec<&str>> { many0(direction)(i) }

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Tile {
    x: i8,
    y: i8,
    z: i8
}


fn is_neighbor(a: Tile, b: Tile) -> bool {
    vec![(a.x-b.x).abs(), (a.y - b.y).abs(), (a.z - b.z).abs()].iter().max() == Some(&1)
}

fn main() -> Result<(), Box<std::error::Error>>{
    let filename = "/home/remy/AOC/2020/24/input";
    let data = fs::read_to_string(filename).unwrap();
    
    /* true: white, false: black */
    let mut tiles: HashMap<Tile, bool> = HashMap::new();

    for line in data.lines() {
        let mut start: Tile = Tile {
            x: 0,
            y: 0,
            z: 0
        };
        for direction in position(line).unwrap().1 {
            match direction {
                "e" => { start.x += 1; start.y -= 1; }
                "se" => { start.y -= 1; start.z += 1 }
                "sw" => { start.x -=1; start.z += 1 }
                "w" => { start.x -= 1; start.y += 1 }
                "nw" => { start.y += 1; start.z -= 1; }
                "ne" =>  { start.x += 1; start.z -= 1; }
                _ => panic!("Wrong input data")
            }
        }
        let mut v = tiles.entry(start).or_insert(true);
        if *v {
            *v = false;
        } else {
            *v = true;
        }
    }

    let mut black_count = 0;
    for (_, tile) in &tiles {
        if !tile {
            black_count += 1;
        }
    }

    println!("Part 1: {}", black_count);

    for i in {0..100} {
        let tiles_copy = tiles.clone();
        let mut new_tiles: HashMap<Tile, u8> = HashMap::new();
        for (tile, value) in &tiles_copy {
            let mut black_count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if (dx == 0 && dy == 0 && dz == 0) || (dx + dy + dz != 0) {
                            continue;
                        }
                        let neighbor: Tile = Tile {
                            x: tile.x + dx,
                            y: tile.y + dy,
                            z: tile.z + dz
                        };
                        if !is_neighbor(neighbor, *tile) {
                            continue;
                        }
                        match (tiles_copy.get(&neighbor), value) {
                            (None, false) => {
                                if !tiles_copy.contains_key(&neighbor) {
                                    *new_tiles.entry(neighbor).or_insert(0) += 1;
                                }
                            }
                            (Some(true), _) => {
                                if !value {
                                    if !tiles_copy.contains_key(&neighbor) {
                                        *new_tiles.entry(neighbor).or_insert(0) += 1;
                                    }
                                }
                            },
                            (Some(false), _) => {
                                black_count += 1;
                                if !value {
                                    if !tiles_copy.contains_key(&neighbor) {
                                        *new_tiles.entry(neighbor).or_insert(0) += 1;
                                    }
                                }
                            }
                            (a, b) => { }
                        }
                    }
                }
            }
            match (value, black_count) {
                (false, i) if i == 0 || i > 2 => { tiles.insert(*tile, true); },
                (true, 2) => { tiles.insert(*tile, false); },
                (_, _) => {}
            }
        }

        for (tile, black_count) in new_tiles {
            if !tiles_copy.contains_key(&tile) {
                if black_count == 2 {
                    tiles.insert(tile, false);
                }
            }
        }
    }

    let mut iblack_count = 0;
    for (_, tile) in &tiles {
        if !tile {
            iblack_count += 1;
        }
    }
    println!("Part 2: {}", iblack_count);

    Ok(())
}
