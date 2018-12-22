extern crate regex;

use regex::RegexSet;
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("running...");

    let contents = parse_input("input".to_string());
    println!("input read...");

    println!("solving...");
    let part_one_solution = solve_part_one(&contents);
    let part_one_solution = solve_part_two(&contents);

    println!("done.\n");
    println!("checksum: {}", part_one_solution);
    println!("box_id: {}", part_one_solution);
}

fn parse_input(filename: String) -> String {
    fs::read_to_string(filename)
        .expect("error reading file\n")
}

fn solve_part_one(contents: &String) -> i32 {
    let ids = contents.split('\n');

    let mut twos: i32 = 0;
    let mut threes: i32 = 0;

    for id in ids {
        let mut id_stats: HashMap<char, i32> = HashMap::new();

        for ch in id.chars() {
            *id_stats.entry(ch).or_insert(0) += 1;
        }

        if id_stats.values().any(|&x| x == 2) {
            twos += 1;
        }

        if id_stats.values().any(|&x| x == 3) {
            threes += 1;
        }
    }

    twos * threes
}

fn solve_part_two(contents: &String) -> String {
    let ids: Vec<&str> = contents.split('\n').collect();
    let num_ids = ids.len();

    for i in 0..num_ids {
        let id_one = ids[i];
        let id_one_length = id_one.len();
        let mut regex_ids: Vec<String> = Vec::new();
        for i in 0..id_one_length {
            let front: String = id_one.chars().skip(0).take(i).collect();
            let _middle: String = id_one.chars().skip(i).take(1).collect();
            let back: String = id_one.chars().skip(i+1).take(id_one_length).collect();
            let string  = format!("{}[a-z]{}", front, back);
            regex_ids.push(string);
        }
        let test_regexes = RegexSet::new(regex_ids).unwrap();

        for j in i..num_ids {
            let id_two = ids[j];
            let matches: Vec<_> = test_regexes.matches(id_two).into_iter().collect();
            if matches.len() == 1 {
                let mismatch = matches[0];
                let front: String = id_one.chars().skip(0).take(mismatch).collect();
                let back: String = id_one.chars().skip(mismatch + 1).take(id_one_length).collect();
                let solution = format!("{}{}", front, back);
                return solution;
            }
        }
    }

    "matching boxes not found".to_string()
}