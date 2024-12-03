use std::io::{self, BufRead};

fn is_valid(nums: &Vec<u64>) -> bool {
    let mut is_ascending: Option<bool> = None;
    let mut is_valid: bool = true;

    for window in nums.windows(2) {
        if let [num1, num2] = window {
            let diff = num1.abs_diff(*num2);

            if diff < 1 || diff > 3 {
                is_valid = false;
                break;
            }

            match is_ascending {
                Some(x) if (num1 < num2) != x => {
                    is_valid = false;
                    break;
                }
                None => is_ascending = Some(num1 < num2),
                _ => {}
            }
        }
    }

    is_valid
}

pub fn second_star() {
    let mut count: u64 = 0;

    for line in io::stdin().lock().lines() {
        match line {
            Ok(str) => {
                let nums: Vec<u64> = str
                    .split_whitespace()
                    .map(|num| num.parse().expect("couldnt parse str to u64"))
                    .collect();

                match is_valid(&nums) {
                    true => {
                        count += 1;
                    }
                    false => {
                        for index in 0..nums.len() {
                            let mut nums2 = nums.clone();
                            nums2.remove(index);

                            if is_valid(&nums2) {
                                count += 1;
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("There was an error getting a line from stdin, {}", err);
            }
        }
    }

    println!("{}", count);
}
