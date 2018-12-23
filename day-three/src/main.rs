extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

struct Claim {
    id: i32,
    xrange: (i32, i32),
    yrange: (i32, i32),
}

fn main() {
    let part_one_solution = solve_part_one();
    println!("Part one solution: {}", part_one_solution);

    let part_two_solution = solve_part_two();
    println!("Part two solution: {}", part_two_solution);
}

fn solve_part_two() -> i32 {
    let f = File::open("./inputs").unwrap();
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims: Vec<_> = reader.by_ref().lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();
            Claim {
                id: cap[1].parse().unwrap(),
                xrange: (cap[2].parse().unwrap(), cap[4].parse().unwrap()),
                yrange: (cap[3].parse().unwrap(), cap[5].parse().unwrap()),
            }
        })
        .collect();

    let mut fabric = HashMap::new();
    let mut ids = HashSet::new();
    for i in 1..1323 {
        ids.insert(i);
    }
    let mut intersected = HashSet::new();

    for claim in &claims {
        for x in (claim.xrange.0)..(claim.xrange.0 + claim.xrange.1)  {
            for y in (claim.yrange.0)..(claim.yrange.0 + claim.yrange.1)  {
                let value = fabric.entry((x,y)).or_insert((0, claim.id));
                (*value).0 += 1;
                if (*value).0 > 1 {
                    intersected.insert(claim.id);
                    intersected.insert((*value).1);
                    (*value).1 = claim.id;
                }
            }
        }
    }

    if ids.difference(&intersected).count() == 1 {
        return ids.difference(&intersected).nth(0).unwrap().clone()
    }

    0
}

fn solve_part_one() -> i32 {
    let f = File::open("./inputs").unwrap();
    let mut reader = BufReader::new(f);

    let mut overlap = 0;

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims: Vec<_> = reader.by_ref().lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();
            Claim {
                id: cap[1].parse().unwrap(),
                xrange: (cap[2].parse().unwrap(), cap[4].parse().unwrap()),
                yrange: (cap[3].parse().unwrap(), cap[5].parse().unwrap()),
            }
        })
        .collect();

    let mut fabric = HashMap::new();

    for claim in &claims {
        for x in (claim.xrange.0)..(claim.xrange.0 + claim.xrange.1)  {
            for y in (claim.yrange.0)..(claim.yrange.0 + claim.yrange.1)  {
                let value = fabric.entry((x,y)).or_insert(0);
                *value += 1;
            }
        }
    }

    for dot in fabric {
        if dot.1 > 1 { overlap += 1 }
    }

    overlap
}
