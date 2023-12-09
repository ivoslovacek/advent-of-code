use std::fs::read_to_string;

fn main() {
    let mut lines: Vec<String> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        lines.push(line.to_string());
    }

    let numbers_str: [String; 9] = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let mut numbers: Vec<u32> = Vec::new();

    for mut line in lines {
        let mut first_number: u32 = 0;
        let mut _last_number: u32 = 0;

        let mut line2 = line.clone();

        replace_in_str(&numbers_str.to_vec(), &mut line);
        reverse_replace_in_str(&numbers_str.to_vec(), &mut line2);

        println!("{}", line);

        for c in line.chars() {
            if c.is_numeric() {
                let number: u32 = c.to_digit(10).unwrap();

                if first_number == 0 {
                    first_number = number;
                }
            }
        }

        for c in line2.chars() {
            if c.is_numeric() {
                let number: u32 = c.to_digit(10).unwrap();

                _last_number = number;
            }
        }

        numbers.push(first_number * 10 + _last_number);
    }

    let sum: u32 = numbers.iter().sum();

    println!("{}", sum);
}

fn replace_in_str(patterns: &Vec<String>, line: &mut String) {
    for i in 0..line.len() {
        for k in 0..patterns.len() {
            if let Some(substr) = line.get(i..i + patterns[k].len()) {
                if substr == patterns[k] {
                    line.replace_range(i..i + patterns[k].len(), (k + 1).to_string().as_str());
                }
            }
        }
    }
}

fn reverse_replace_in_str(patterns: &Vec<String>, line: &mut String) {
    for i in (0..line.len()).rev() {
        for k in 0..patterns.len() {
            if let Some(substr) = line.get(i..i + patterns[k].len()) {
                if substr == patterns[k] {
                    line.replace_range(i..i + patterns[k].len(), (k + 1).to_string().as_str());
                }
            }
        }
    }
}
