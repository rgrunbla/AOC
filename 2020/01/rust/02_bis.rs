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

        for i in 0..(stack.len()-1) {
            let mut start = i;
            let mut end = stack.len()-1;

            loop {
                if start > end {
                    break;
                }
                let front = stack[start];
                let back = stack[end];
                if stack[i] + front + back == 2020 {
                    println!("{}", stack[i] * front * back);
                    return;
                } else if stack[i] + front + back > 2020 {
                    end -= 1;
                } else {
                    start+=1;
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
