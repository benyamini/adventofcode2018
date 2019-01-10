extern crate regex;
extern crate chrono;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;
use regex::Regex;
use chrono::NaiveDateTime;
use chrono::Duration;
use chrono::Timelike;

struct Day {
    activities: Vec<Activity>,
    _guard_id: u32,
    _start_time: NaiveDateTime,
    _end_time: NaiveDateTime,
    _shift: Vec<String>,
}

impl Day {
    fn new(_guard_id: u32, _start_time: NaiveDateTime) -> Day {
        Day {
            activities: Vec::default(),
            _guard_id,
            _start_time,
            _end_time: {
                if _start_time.format("%H").to_string() == "23" {
                    let corrected_start_date = _start_time.checked_add_signed(Duration::hours(1)).expect("error adding time");
                    let formatted_date = format!("1518{}{}00:59", corrected_start_date.format("%m").to_string(), corrected_start_date.format("%d").to_string());
                    NaiveDateTime::parse_from_str(formatted_date.as_str(), "%Y%m%d%R").expect("error starting day")
                } else {
                    let formatted_date = format!("1518{}{}00:59", _start_time.format("%m").to_string(), _start_time.format("%d").to_string());
                    NaiveDateTime::parse_from_str(formatted_date.as_str(), "%Y%m%d%R").expect("error starting day")
                }
            },
            _shift: {
                let mut new_shift = Vec::new();
                for _i in 0..60 {
                    new_shift.push(".".to_string());
                }
                new_shift
            }
        }
    }

    fn print_day(&self) {
        println!("guard_id: {}", self._guard_id);
        println!("start_time: {}", self._start_time);
        println!("end_time: {}\n", self._end_time);
        for a in &self.activities {
            a.print_activity();
        }
        print!("shift: ");
        for m in 0..60 {
            print!("{}", self._shift[m])
        }
        print!("\n\n");
    }

    fn calculate_sleep_time(&mut self) {
        for activity in &self.activities {
            if activity.activity == "falls asleep".to_string() {
                let minute = activity.date_time.minute();
                for i in minute..60 {
                    let val = &mut self._shift[i as usize];
                    *val = "x".to_string();
                }
            }

            if activity.activity == "wakes up".to_string() {
                let minute = activity.date_time.minute();
                for i in minute..60 {
                    let val = &mut self._shift[i as usize];
                    *val = ".".to_string();
                }
            }
        }
    }
}

#[derive (Debug)]
struct Activity {
    date_time: NaiveDateTime,
    activity: String,
}

impl Activity {
    fn new(naive_date_time: NaiveDateTime, activity: String) -> Activity {
        Activity { date_time: naive_date_time, activity  }
    }

    fn print_activity(&self) {
        println!("date: {}", self.date_time);
        println!("activity: {}\n", self.activity);
    }
}

fn main() {


    let f = File::open("./inputs").unwrap();
    let mut reader = BufReader::new(f);

    let mut lines: Vec<Activity> = Vec::new();
    let line_regex = Regex::new(r"\[(?P<year>[\d]+)-(?P<month>[\d]+)-(?P<day>[\d]+)\s(?P<time>[\d:]+)]\s(?P<activity>[a-zA-Z\s#0-9]*)").unwrap();

    for l in reader.by_ref().lines() {
        let line = l.unwrap();
        let line_ref = line.trim().as_ref();
        let caps = line_regex.captures_iter(line_ref);
        for cap in caps {
            let date_time_string =  format!("{}{}{}{}", &cap["year"], &cap["month"], &cap["day"], &cap["time"]);
            let date = NaiveDateTime::parse_from_str(date_time_string.as_str(), "%Y%m%d%R").expect("Failed to parse date");
            let activity = Activity::new(date, (&cap["activity"]).parse().unwrap());
            lines.push(activity);
        }
    }
    lines.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    let mut days : Vec<Day> = Vec::new();

    let guard_regex = Regex::new(r"(Guard #)(?P<guard_id>\d*)").unwrap();
    let sleep_regex = Regex::new(r"(falls asleep)|(wakes up)").unwrap();

    let mut lines_iter = lines.iter();

    while let Some(l) = lines_iter.next() {
        let guard_caps = guard_regex.captures_iter(&l.activity);
        for guard_cap in guard_caps {
            let guard_id = guard_cap["guard_id"].parse::<u32>().unwrap();
            let mut new_day = Day::new(guard_id, l.date_time);

            while let Some(n) = lines_iter.next() {
                match guard_regex.captures(&n.activity) {
                    Some(_x) => break,
                    None => {
                        let mut sleep_caps = sleep_regex.captures_iter(&n.activity);
                        while let Some(c) = sleep_caps.next() {
                            let new_sleep_activity = Activity::new(n.date_time, c.get(0).unwrap().as_str().to_string());
                            new_day.activities.push(new_sleep_activity);
                        }
                    },
                }
            }
            days.push(new_day);
        }
    }

    for day in &mut days {
        day.calculate_sleep_time();
        day.print_day();
    }
}
