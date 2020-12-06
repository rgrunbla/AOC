use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let filename = "../input";

    let mut stack = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                stack.push(number.parse::<i32>().unwrap());
            }
        }
        stack.sort();
        println!("{:?}", stack);
        for number1 in stack.iter() {
            for number2 in stack.iter() {
                if number1+number2 == 2020 {
                    println!("{}", number1*number2);
                }
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}