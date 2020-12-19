extern crate nom;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, space0},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
  };
  
  use std::str::FromStr;
  
  /* Expression dans des parenthèses, avec zéro ou plusieurs espaces autour */
  fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
  }
  
  /* Entier, avec zéro ou plusieurs espaces autour, ou expression dans des parenthèses */
  fn factor(i: &str) -> IResult<&str, i64> {
    alt((
      map_res(delimited(space0, digit1, space0), FromStr::from_str),
      parens,
    ))(i)
  }

  /* Facteur suivi d'autres facteurs séparés chacun par un '+' */
  fn term(i: &str) -> IResult<&str, i64> {
      
    /* init: premier facteur, i: reste de la chaine */
    let (i, init) = factor(i)?;
  
    fold_many0(
      pair(char('+'), factor),
      init,
      |acc, (op, val): (char, i64)| {
        match op {
            '+' => acc + val,
            _ => panic!("Unknown op")
        }
      },
    )(i)
  }

  fn expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = term(i)?;
  
    fold_many0(
      pair(char('*'), term),
      init,
      |acc, (op, val): (char, i64)| {
            match op {
                '*' => acc * val,
                _ => panic!("Unknown op")
            }
        })(i)
  }

fn main() {
    let filename = "/home/remy/AOC/2020/18/input";
    let mut res: i64 = 0;

    if let Ok(lines) = read_lines(filename) {
        for row in lines {
            if let Ok(row) = row {
                let (_, i) = expr(&row).unwrap();
                res += i;
            }
        }
        println!("Result: {}", res);
    } else {
        println!("Error");
    }
}
