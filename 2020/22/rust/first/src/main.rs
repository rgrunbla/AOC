extern crate nom;
use std::fmt;
use std::fs;
use std::collections::{VecDeque, HashMap, HashSet};
use std::iter::FromIterator;

struct Player {
    number: usize,
    cards: VecDeque<usize>
}

impl Default for Player {
    fn default() -> Self {
        Player {
            number: 0,
            cards: VecDeque::<usize>::new()
        }
    }
}

impl Player {
    fn score(self) -> usize  {
        let mut factor = self.cards.len();
        let mut res = 0;
        for item in self.cards {
            res += item*factor;
            factor -= 1;
        }
        res
    }
}

fn Play(p1: &mut Player, p2: &mut Player) -> bool {
    match (p1.cards.len(), p2.cards.len()) {
        (0, _) => true,
        (_, 0) => true,
        (_, _) => match (p1.cards.front(), p2.cards.front()) {
                (Some(i), Some(j)) if i<j => {
                    println!("Player 1 plays {}", i);
                    println!("Player 2 plays {}", j);
                    println!("Player 1 wins");
                    p2.cards.push_back(*j);
                    p2.cards.push_back(*i);
                    p2.cards.pop_front();
                    p1.cards.pop_front();
                    false
                },
                (Some(i), Some(j)) if j<i => {
                    println!("Player 1 plays {}", i);
                    println!("Player 2 plays {}", j);
                    println!("Player 2 wins");
                    p1.cards.push_back(*i);
                    p1.cards.push_back(*j);
                    p2.cards.pop_front();
                    p1.cards.pop_front();
                    false
                },
                (_, _) => panic!("wat")
            
        }

    }
}

fn main() -> Result<(), Box<std::error::Error>>{
    let filename = "/home/remy/AOC/2020/22/input";
    let data = fs::read_to_string(filename).unwrap();

    let mut p1: Player = Player::default();
    let mut p2: Player = Player::default();


    for (i, player) in data.split("\n\n").enumerate() {
        let mut splitted = player.split("\n");
        let player_line = splitted.next().unwrap();
        let player_number: String = player_line[7..player_line.len()-1].to_string();

        println!("Player Number {}", player_number);

        let mut cards:  VecDeque<usize> = VecDeque::new();
        for card in splitted {
            println!("{}", card);
            cards.push_back(card.parse::<usize>().unwrap());
        }

        if i == 0 {
            p1 = Player {
                number: player_number.parse::<usize>().unwrap(),
                cards: cards
            }
        } else {
            p2 = Player {
                number: player_number.parse::<usize>().unwrap(),
                cards: cards
            }
        }
    }


    while(!Play(&mut p1, &mut p2)) {}
    println!("{}", p1.score());
    println!("{}", p2.score());

    Ok(())
}
