use std::fs;
use std::collections::{VecDeque, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone)]
struct Player {
    number: u32,
    cards: VecDeque<u32>
}

impl Default for Player {
    fn default() -> Self {
        Player {
            number: 0,
            cards: VecDeque::<u32>::new()
        }
    }
}

impl Player {
    fn score(self) -> u32  {
        let mut factor = self.cards.len() as u32;
        let mut res = 0;
        for item in self.cards {
            res += item*factor;
            factor -= 1;
        }
        res
    }
}

fn calculate_hash(t: (&VecDeque<u32>, &VecDeque<u32>)) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn play(p1: &mut Player, p2: &mut Player, played: &mut HashSet<u64>) -> (bool, bool) {
    println!("P1 deck: {:?}\nP2 deck: {:?}", p1.cards, p2.cards);
    if played.contains(&calculate_hash((&p1.cards, &p2.cards))) {
        let i = p1.cards.pop_front().unwrap();
        let j = p2.cards.pop_front().unwrap();
        p1.cards.push_back(j);
        p1.cards.push_back(i);
        return (true, true)
    } else {
        played.insert(calculate_hash((&p1.cards, &p2.cards)));
    }
    if p1.cards.len() > 0 && p2.cards.len() > 0 {
        if p1.cards.iter().max().unwrap() > p2.cards.iter().max().unwrap() && p1.cards.iter().max().unwrap() > &((p1.cards.len()+p2.cards.len()) as u32) {
            return (true, true);
        }
    }

    let i = p1.cards.pop_front();
    let j = p2.cards.pop_front();
    match (i, j) {
        (Some(k), Some(l)) if (k <= p1.cards.len() as u32 && l <= p2.cards.len() as u32) => {

            let mut pi1 = p1.clone();
            let mut pi2 = p2.clone();
            while pi1.cards.len() as u32 != k {
                pi1.cards.pop_back();
            }
            while pi2.cards.len() as u32 != l {
                pi2.cards.pop_back();
            }

            let mut played: HashSet<u64> = HashSet::new();
            loop {
                let (finished, p1wins) = play(&mut pi1, &mut pi2, &mut played);
                if finished {
                    if p1wins {
                        p1.cards.push_back(k);
                        p1.cards.push_back(l);
                    } else {
                        p2.cards.push_back(l);
                        p2.cards.push_back(k);
                    }
                    return (false, p1wins);
                }
            }
        },
        (Some(k),Some(l)) if k > l => {
            p1.cards.push_back(k);
            p1.cards.push_back(l);
            (false, true)
        },
        (Some(k),Some(l)) if k < l => {
            p2.cards.push_back(l);
            p2.cards.push_back(k);
            (false, false)
        }
        (Some(_),Some(_)) => {
            panic!("equality ?!?");
        },
        (None,_) => {
            p2.cards.push_front(j.unwrap());
            return (true, false);
        },
        (_, None) => {
            p1.cards.push_front(i.unwrap());
            return (true, true)
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>>{
        let filename = "/home/remy/AOC/2020/22/input";
        let data = fs::read_to_string(filename).unwrap();
    
        for data in data.split("\n\n\n") {

            println!("\n\nData: {:?}\n\n", data);

        let mut p1: Player = Player::default();
        let mut p2: Player = Player::default();


        for (i, player) in data.split("\n\n").enumerate() {
            let mut splitted = player.split("\n");
            let player_line = splitted.next().unwrap();
            let player_number: String = player_line[7..player_line.len()-1].to_string();

            let mut cards:  VecDeque<u32> = VecDeque::new();
            for card in splitted {
                cards.push_back(card.parse::<u32>().unwrap());
            }

            if i == 0 {
                p1 = Player {
                    number: player_number.parse::<u32>().unwrap(),
                    cards: cards
                }
            } else {
                p2 = Player {
                    number: player_number.parse::<u32>().unwrap(),
                    cards: cards
                }
            }
        }

        loop {
            let mut played:HashSet<u64> = HashSet::new();
            let (finished, _) = play(&mut p1, &mut p2, &mut played);
            if finished {
                break;
            }
        }

        let a = p1.score();
        let b = p2.score();
        if a == 0 {
            println!("{},", b);
        } else {
            println!("{},", a);
        }
    }

    Ok(())
}
