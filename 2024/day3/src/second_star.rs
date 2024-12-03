use regex::Regex;
use regex_split::RegexSplit;
use std::{
    i64,
    io::{self, BufRead},
};

pub fn solution() {
    let mut sum: i64 = 0;
    let mut is_ok: bool = true;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let line = line.as_str();
                let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                let reg_split = Regex::new(r"(don't\(\)|do\(\))").unwrap();

                for segment in reg_split.split_inclusive_left(line) {
                    if segment.trim().is_empty() {
                        continue;
                    }

                    if segment.contains("don't()") {
                        is_ok = false;
                    } else if segment.contains("do()") {
                        is_ok = true;
                    }

                    if is_ok {
                        for captures in reg.captures_iter(segment) {
                            let num1 = captures[1].parse::<i64>().unwrap();
                            let num2 = captures[2].parse::<i64>().unwrap();

                            sum += num1 * num2;
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error while reading a line from stdin, {}", err);
            }
        }
    }

    println!("{}", sum);
}
