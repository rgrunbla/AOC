extern crate nom;

use std::fs;
use std::collections::HashMap;

pub enum Either<L, M, R> {
    Left(L),
    Middle(M),
    Right(R),
}

use nom::{
    branch::alt,
    character::complete::char,
    character::complete::{digit1, space0, anychar},
    combinator::map_res,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
  };
  
  use std::str::FromStr;
  
  /* Single Rule */
  fn factor(i: &str) -> IResult<&str, i64> {
      map_res(delimited(space0, digit1, space0), FromStr::from_str)(i)
  }

  fn term(i: &str) -> IResult<&str, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> {
    many0(factor)
    (i).map(|(next_input, res)| (next_input, Either::Middle(res)))
  }

  fn unique_char(i: &str) -> IResult<&str, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> {
    delimited(space0, delimited(char('"'), anychar, char('"')), space0)
    (i).map(|(next_input, res)| (next_input, Either::Left(res)))
  }

  fn simple_rule(i: &str) -> IResult<&str, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> {
      alt(
        (
            unique_char,
            term
        )
    )(i)
  }

  fn complex_rule(i: &str) -> IResult<&str, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> {
    separated_pair(term, char('|'), term)
    (i)
    .map(|(next_input, res)| {
        match res {
            (Either::Middle(left), Either::Middle(right)) => (next_input, Either::Right((left, right))),
            _ => panic!("{}")
        }
    })
  }

  fn expr(i: &str) -> IResult<&str, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> {
        alt(
            (
                complex_rule,
                simple_rule
            )
        )(i)
  }

fn is_valid<'a>(line: &'a str, idx: &[i64], rules: &HashMap<i64, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>>) -> bool {
    match idx.len() {
        0 => {
            return line.len() == 0
        },
        _ => {
            //println!("Rule {}", idx[0]);
            match &rules[&idx[0]] {
                Either::Left(car) => {
                    return match line.len() {
                        0 => false,
                        i => match line.chars().nth(0) {
                                Some(x) if x == *car => {
                                    is_valid(&line[1..], &idx[1..], rules)
                                },
                                _ => false
                            }
                    }
                },
                Either::Right((first, second)) => {
                    return is_valid(&line, &[&first[..], &idx[1..]].concat(), rules) ||
                           is_valid(&line, &[&second[..], &idx[1..]].concat(), rules);
                },
                Either::Middle(mid) => {
                    return is_valid(&line, &[&mid[..], &idx[1..]].concat(), rules)
                }
            }
        }
    }
}

fn main() {
    let filename = "/home/remy/AOC/2020/19/input";
    let mut rules_map: HashMap<i64, Either<char, Vec<i64>, (Vec<i64>, Vec<i64>)>> = HashMap::new();

    let data = fs::read_to_string(filename).unwrap();
    let data: Vec<&str> = data.split("\n\n").collect();
    let rules = data[0];
    let data = data[1];
    for rule in rules.lines() {
        let mut tokens = rule.split(":");
        let number = &tokens.next().unwrap();
        let number = number.parse::<i64>().unwrap();
        let rule = &tokens.next().unwrap().trim();
        println!("{} -> {}", number, rule);
        match expr(rule) {
            Ok((_, foo)) => {
                rules_map.insert(number, foo);
            },
            Err(a) => {
                panic!("wat");
            }
        }
    }

    let mut valid: i64 = 0;
    for line in data.lines() {
        if is_valid(line, &vec![0], &rules_map) {
            valid += 1;
        }
    }
    println!("Valids: {}", valid);
}
