use regex::Regex;
use std::{
    i64,
    io::{self, BufRead},
};

pub fn solution() {
    let mut sum: i64 = 0;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

                for captures in regex.captures_iter(line.as_str()) {
                    let num1 = captures[1].parse::<i64>().unwrap();
                    let num2 = captures[2].parse::<i64>().unwrap();

                    sum += num1 * num2;
                }
            }
            Err(err) => {
                println!("Error while reading a line from stdin, {}", err);
            }
        }
    }

    println!("{}", sum);
}
