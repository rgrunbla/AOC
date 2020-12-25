use std::fs;

fn transform(mut value: i64, subject_number: i64) -> i64 {
    value *= subject_number;
    value = value % 20201227;
    value
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let filename = "/home/remy/AOC/2020/25/input";
    let data = fs::read_to_string(filename).unwrap();
    let mut splitted = data.split("\n");
    let pub1 = splitted.next().unwrap().parse::<i64>().unwrap();
    let pub2 = splitted.next().unwrap().parse::<i64>().unwrap();
    println!("Pub 1: {}", pub1);
    println!("Pub 2: {}", pub2);

    let mut t1 = 7;
    let mut i1 = 1;
    loop {
        if t1 == pub1 {
            break;
        }
        t1 = transform(t1, 7);
        i1+=1;
    }
    println!("{} {} {}", pub1, t1, i1);
    
    let mut result = 1;
    for _ in 0..i1 {
        result = transform(result,pub2);

    }

    println!("result: {}", result);

    Ok(())
}
