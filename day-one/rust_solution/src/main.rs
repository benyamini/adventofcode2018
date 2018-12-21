use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = parse_input("inputs".to_string());
    let mut counted: bool = false;
    let mut count: i32 = 0;

    let mut values: HashMap<i32, i32> = HashMap::new();
    let mut twice_counted: bool = false;

    while !twice_counted {
        for line in contents.split('\n') {
            let value = String::from(line);
            if value.len() > 1 {
                if value.starts_with('+') {
                    count += value[1..].parse::<i32>().unwrap();
                } else {
                    count -= value[1..].parse::<i32>().unwrap();
                }

                let value_count = values.entry(count).or_insert(0);
                *value_count += 1;
                if *value_count == 2 {
                    twice_counted = true;
                    println!("{}", count);
                    break;
                }
            }
        }
        if !counted {
            println!("{}", count);
            counted = true;
        }
    }
}

fn parse_input(filename: String) -> String {
    fs::read_to_string(filename)
        .unwrap()
}