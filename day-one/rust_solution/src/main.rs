use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = parse_input("inputs".to_string());
    let (total, double) = solve(contents);

    match total {
        Some(x) => println!("Total: {}", x),
        None => (),
    };

    match double {
        Some(x) => println!("Found twice: {}", x),
        None => (),
    };
}

fn parse_input(filename: String) -> String {
    fs::read_to_string(filename)
        .expect("error reading file\n")
}

fn solve(contents: String) -> (Option<i32>, Option<i32>) {
    let mut counted: bool = false;
    let mut count: i32 = 0;
    let mut final_count: Option<i32> = None;

    let mut values: HashMap<i32, i32> = HashMap::new();
    let mut twice_counted: bool = false;
    let mut twice_counted_value: Option<i32> = None;

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
                    twice_counted_value = Some(count);
                    break;
                }
            }
        }
        if !counted {
            counted = true;
            final_count = Some(count);
        }
    }

    (final_count, twice_counted_value)
}