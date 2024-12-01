use std::{
    i64,
    io::{self, BufRead},
};

pub fn first_star() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let nums: Vec<i64> = input
                    .split_whitespace()
                    .map(|num| num.parse().expect("couldnt parse number"))
                    .collect();

                if nums.len() != 2 {
                    eprintln!("Less then 2 numbers on a line");
                    break;
                }

                list1.push(nums[0]);
                list2.push(nums[1]);
            }
            Err(err) => {
                eprintln!("Error reading from input {}", err);
                break;
            }
        }
    }

    list1.sort();
    list2.sort();

    let sorted_pairs: Vec<(i64, i64)> = list1.into_iter().zip(list2.into_iter()).collect();

    for (num1, num2) in sorted_pairs {
        sum += num1.abs_diff(num2);
    }

    println!("{}", sum);
}
