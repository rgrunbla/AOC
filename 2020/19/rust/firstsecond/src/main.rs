extern crate nom;

use nohash_hasher::IntMap;

use std::fs;
//use std::collections::IntMap;

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
  fn factor(i: &str) -> IResult<&str, u16> {
      map_res(delimited(space0, digit1, space0), FromStr::from_str)(i)
  }

  fn term(i: &str) -> IResult<&str, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> {
    many0(factor)
    (i).map(|(next_input, res)| (next_input, Either::Middle(res)))
  }

  fn unique_char(i: &str) -> IResult<&str, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> {
    delimited(space0, delimited(char('"'), anychar, char('"')), space0)
    (i).map(|(next_input, res)| (next_input, Either::Left(res)))
  }

  fn simple_rule(i: &str) -> IResult<&str, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> {
      alt(
        (
            unique_char,
            term
        )
    )(i)
  }

  fn complex_rule(i: &str) -> IResult<&str, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> {
    separated_pair(term, char('|'), term)
    (i)
    .map(|(next_input, res)| {
        match res {
            (Either::Middle(left), Either::Middle(right)) => (next_input, Either::Right((left, right))),
            _ => panic!("{}")
        }
    })
  }

  fn expr(i: &str) -> IResult<&str, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> {
        alt(
            (
                complex_rule,
                simple_rule
            )
        )(i)
  }

fn is_valid<'a>(line: &'a str, idx: &[u16], rules: &IntMap<u16, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>>) -> bool {
    match idx.len() {
        0 => {
            return line.len() == 0
        },
        _ => {
            match &rules[&idx[0]] {
                Either::Left(car) => {
                    return match line.len() {
                        0 => false,
                        _ => match line.chars().nth(0) {
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
    let mut rules_map: IntMap<u16, Either<char, Vec<u16>, (Vec<u16>, Vec<u16>)>> = IntMap::default();
    let data = fs::read_to_string(filename).unwrap();
    for rule in data.lines() {
        if rule.is_empty() {
            break;
        }
        let mut tokens = rule.split(":");
        let number = &tokens.next().unwrap().parse::<u16>().unwrap();
        let rule = &tokens.next().unwrap().trim();
        match expr(rule) {
            Ok((_, foo)) => {
                rules_map.insert(*number, foo);
            },
            Err(_) => panic!("wat")
        }
    }

    let mut valid: u16 = 0;
    for line in data.lines() {
        if is_valid(line, &vec![0], &rules_map) {
            valid += 1;
        }
    }
    println!("Valids: {}", valid);
}
