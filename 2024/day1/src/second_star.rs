use std::{
    collections::HashMap,
    i64,
    io::{self, BufRead},
};

fn get_frequencies<T: std::hash::Hash + Eq>(vec: &Vec<T>) -> HashMap<&T, usize> {
    let mut frequencies: HashMap<&T, usize> = HashMap::new();
    for item in vec {
        *frequencies.entry(item).or_insert(0) += 1;
    }
    frequencies
}

pub fn second_star() {
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

    let frequencies = get_frequencies(&list2);

    for item in list1 {
        let frequency = *frequencies.get(&item).unwrap_or(&0);
        sum += (item as u64) * (frequency as u64);
    }

    println!("{}", sum);
}
