extern crate nom;
use std::fmt;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    let filename = "/home/remy/AOC/2020/21/input";
    let data = fs::read_to_string(filename).unwrap();
    let mut potential_matching: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients: HashMap<String, isize> = HashMap::new();

    for line in data.split("\n") {
        let mut splitted = line.split("(");
        let mut ingredients_list: Vec<String>  = Vec::new();
        
        for ingredient in splitted.next().unwrap().trim().split(" ") {
            ingredients_list.push(ingredient.to_string());
            *ingredients.entry(ingredient.to_string()).or_insert(0) += 1;
        }

        let mut allergens_list: Vec<String> = Vec::new();
        match splitted.next() {
            Some(x) => {
                for allergen in x[9..x.len()-1].trim().split(", ") {
                    allergens_list.push(allergen.to_string());
                }
            },
            _ => ()
        }
        for allergen in &allergens_list {
            let potential_matching = potential_matching.entry(allergen.to_string()).or_insert(HashSet::from_iter(ingredients_list.iter().cloned()));
            let mut to_remove = Vec::new();
            for ingredient in potential_matching.iter() {
                if !ingredients_list.contains(&ingredient) {
                    to_remove.push(ingredient.clone());
                }
            }
            for item in &to_remove {
                potential_matching.remove(item);
            }
        }
    }

    let mut has_allergens = HashSet::new();
    let mut correspondance: Vec<(String, String)> = Vec::new();

    loop {
        let mut to_remove: Vec<String> = Vec::new();
        for (allergen, ingredients) in potential_matching.iter() {
            if ingredients.len() == 1 {
                to_remove.push(ingredients.iter().next().unwrap().to_string());
                correspondance.push((allergen.to_string(), to_remove[to_remove.len()-1].clone()));
                has_allergens.insert(to_remove[to_remove.len()-1].clone());
            }
        }
        if to_remove.len() == 0 {
           break;
        }

        
        for item in to_remove {
            for (_, ingredients) in potential_matching.iter_mut() {
                ingredients.remove(&item);
            }
        }
    }

    let mut res = 0;

    for (ingredient, count) in ingredients {
        if !has_allergens.contains(&ingredient) {
            res += count;
        }
    }

    println!("Result 1: {}", res);

    correspondance.sort();
    for item in correspondance {
        println!("{:?}", item.1);
    }
    
}
