extern crate nom;
use std::fmt;
use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Tile {
    top: String,
    right: String,
    bottom: String,
    left: String,
    data: [[char; 8]; 8]
}

fn print_tile(tile: &Tile) {
    println!("\n{}", tile.top);
    for i in {0..8} {
        print!("{}", tile.left.chars().nth(1+i).unwrap());
        print!("{}", tile.data[i].iter().collect::<String>());
        print!("{}", tile.right.chars().nth(1+i).unwrap());
        println!("");
    } 
    println!("{}\n", tile.bottom);
}

fn vs_tile(tile: Tile) -> Tile {
    let mut new_data: [[char; 8]; 8] = Default::default();
    for i in 0..8 {
        for j in 0..8 {
            new_data[i][j] = tile.data[i][7-j];
        }
    }
    Tile{
        top: tile.top.chars().rev().collect(),
        right: tile.left,
        bottom: tile.bottom.chars().rev().collect(),
        left: tile.right,
        data: new_data
    }
}

fn hs_tile(tile: Tile) -> Tile {
    let mut new_data: [[char; 8]; 8] = Default::default();
    for i in 0..8 {
        for j in 0..8 {
            new_data[i][j] = tile.data[7-i][j];
        }
    }
    Tile{
        top: tile.bottom,
        right: tile.right.chars().rev().collect(),
        bottom: tile.top,
        left: tile.left.chars().rev().collect(),
        data: new_data
    }
}

fn r_tile(tile: Tile) -> Tile {
    let mut new_data: [[char; 8]; 8] = Default::default();
    for i in 0..8 {
        for j in 0..8 {
            new_data[i][j] = tile.data[7-j][i];
        }
    }
    Tile{
        top: tile.left.chars().rev().collect(),
        right: tile.top,
        bottom: tile.right.chars().rev().collect(),
        left: tile.bottom,
        data: new_data
    }
}

fn gen_tiles(orig_tile: Tile) -> Vec<Tile> {
    let rot: Vec<Tile> = gen_tiles_rot(orig_tile);
    let mut ret: Vec<Tile> = Vec::new();
    
    for orig_tile in rot {
        let tile = orig_tile.clone();
        ret.push(tile.clone());

        let tile = hs_tile(orig_tile.clone());
        ret.push(tile);

        let tile = vs_tile(orig_tile.clone());
        ret.push(tile);

        let tile = vs_tile(hs_tile(orig_tile.clone()));
        ret.push(tile);
    }

    ret
}

fn gen_tiles_rot(orig_tile: Tile) -> Vec<Tile> {
    let mut ret: Vec<Tile> = Vec::new();

    let tile = r_tile(orig_tile.clone());
    ret.push(tile);
    let tile = r_tile(r_tile(orig_tile.clone()));
    ret.push(tile);
    let tile = r_tile(r_tile(r_tile(orig_tile.clone())));
    ret.push(tile);
    

    ret
}

fn main() {
    let filename = "/home/remy/AOC/2020/20/input";
    let data = fs::read_to_string(filename).unwrap();
    let mut tiles: HashMap<i64, Vec<Tile>> = HashMap::new();
    let mut counts: HashMap<String, HashSet<i64>> = HashMap::new();

    for tile in data.split("\n\n") {
        let lines: Vec<&str> = tile.lines().collect();
        let splitted: Vec<&str> = lines[0].split(' ').collect();
        assert_eq!(splitted[0], "Tile");
        let id = splitted[1].trim_end_matches(':').parse::<i64>().unwrap();

        let top_str = lines[1].trim().to_string();
        let bot_str = lines[lines.len()-1].trim().to_string();

        let mut left_str: String = String::default();
        let mut right_str: String = String::default();

        let mut data: [[char; 8]; 8] = Default::default();


        for i in 0..10 {
            left_str.push(lines[1+i].chars().nth(0).unwrap());
            right_str.push(lines[1+i].chars().nth(9).unwrap());
        }

        for i in 0..8 {
            for j in 0..8 {
                data[i][j] = lines[i+2].chars().nth(j+1).unwrap();
            }
        }
        let left_str = left_str.trim().to_string();
        let right_str = right_str.trim().to_string();

        let tile: Tile = Tile {
            top: top_str,
            right: right_str,
            bottom: bot_str,
            left: left_str,
            data: data
        };

        println!("Tile {}", id);
        print_tile(&tile.clone());
        print_tile(&vs_tile(tile.clone()));

        let generated_tiles = gen_tiles(tile.clone());

        for generated_tile in &generated_tiles {
            counts.entry(generated_tile.top.clone()).or_insert(HashSet::new()).insert(id);
            counts.entry(generated_tile.right.clone()).or_insert(HashSet::new()).insert(id);
            counts.entry(generated_tile.bottom.clone()).or_insert(HashSet::new()).insert(id);
            counts.entry(generated_tile.left.clone()).or_insert(HashSet::new()).insert(id);
        }
        tiles.insert(id, generated_tiles);
    }

    let mut result = 1;
    let mut first_tile : i64 = 0;

    let width = (*&tiles.len() as f64).sqrt() as usize;
    for (i, orientations) in &tiles {
        let mut neighbors: HashSet<i64> = HashSet::new();
        for (k, v) in &counts {
            if v.contains(&i) {
                for i in v {
                    neighbors.insert(*i);
                }               
            }
        }
        if neighbors.len()-1 == 2 {
            first_tile = *i;
            println!("Tile {} has {} neighbors", i, neighbors.len()-1);
            result *= i;
        }
    }

    println!("Result: {}", result);

    let mut grid: Vec<Vec<&Tile>> = Vec::new();
    grid.push(Vec::new());
    let mut i: usize = 0;
    let mut j: usize = 0;

    let (id, generated_first_tiles) = tiles.remove_entry(&first_tile).unwrap();
    for generated_first_tile in &generated_first_tiles {
        if counts[&generated_first_tile.left].len() == 1 && counts[&generated_first_tile.top].len() == 1 {
            grid[0].push(&generated_first_tile);
            println!("Tile {}", id);
            break;
        }
    }

    j += 1;
    let mut already_used: HashSet<i64> = HashSet::new();

    loop {
        if i == width {
            break;
        }

        let mut found = false;

        if j > 0 {
            let left = grid[i][j-1];
            for (tile_id, generated_tiles) in &tiles {
                if already_used.contains(tile_id) {
                    continue;
                }
                for tile in generated_tiles {
                    if tile.left == left.right {
                        if i == 0 {
                            already_used.insert(*tile_id);
                            println!("Tile {}", *tile_id);
                            grid[i].push(tile);
                            found = true;
                            break;
                        } else {
                            let top = grid[i-1][j];
                            if tile.top == top.bottom {
                                already_used.insert(*tile_id);
                                println!("Tile {}", *tile_id);
                                grid[i].push(tile);
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if found {
                    break;
                }
            }
        } else {
            let top = grid[i-1][j];
            for (tile_id, generated_tiles) in &tiles {
                if already_used.contains(tile_id) {
                    continue;
                }
                for tile in generated_tiles {
                    if tile.top == top.bottom {
                        already_used.insert(*tile_id);
                        println!("Tile {}", *tile_id);
                        grid[i].push(tile);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            } 
        }
        j += 1;
        if j == (width) {
            i += 1;
            j = 0;
            grid.push(Vec::new());
        }
    }
    let mut full_grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..width*8 {
        let mut line: Vec<char> = Vec::new();
        for _ in 0..width*8 {
            line.push(' ');
        }
        full_grid.push(line);
    }

    for i in 0..width {
        for j in 0..width {
            for inner_i in 0..8 {
                for inner_j in 0..8 {
                    full_grid[i*8+inner_i][j*8+inner_j] = grid[i][j].data[inner_i][inner_j];
                }
            }
        }
    }
    for (i, line) in full_grid.iter().enumerate() {
        for (j, car) in line.iter().enumerate() {
            print!("{}", car);
        }
        println!("");
    }
    let mut count = 0;
    let mut full_grid_copy = full_grid.clone();
    let sea_monster = vec!["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "];
    for i in 0..(width*8-3) {
        for j in 0..(width*8-20) {
            let mut found = true;
            for (inner_i, line) in sea_monster.iter().enumerate() {
                for (inner_j, car) in line.chars().enumerate() {
                    if car == '#' && !(full_grid[i+inner_i][j+inner_j] == '#') {
                        found = false;
                    }
                }
            }
            if found {
                count += 1;
                for (inner_i, line) in sea_monster.iter().enumerate() {
                    for (inner_j, car) in line.chars().enumerate() {
                        if car == '#' {
                            full_grid_copy[i+inner_i][j+inner_j] = 'O';
                        }
                    }
                }
            }
        }
    }
    println!("Sea monsters: {}", count);
    count = 0;
    for line in full_grid_copy {
        for car in line.iter() {
            if *car == '#' {
                count += 1;
            }
        }
    }
    println!("Harsh: {}", count);
}
