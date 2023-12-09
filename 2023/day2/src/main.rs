use std::fs;

fn main() {
    let mut lines: Vec<String> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        lines.push(line.to_string());
    }

    for i in 0..lines.len() {
        let mut split_line = lines[i].split(": ");
        split_line.next();

        lines[i] = split_line.next().unwrap().to_string();
    }

    let colors: [&str; 3] = ["red", "green", "blue"];

    let mut sum: u32 = 0;

    for i in 0..lines.len() {
        let mut power: u32 = 1;
        let mut minimum_color: [u32; 3] = [0, 0, 0];

        for set in lines[i].split("; ") {
            let mut last_num: u32 = 0;
            let mut colors_numbers: [u32; 3] = [0, 0, 0];

            for set_part in set.split(", ") {
                for part in set_part.split(" ") {
                    match part.parse::<u32>() {
                        Ok(n) => {
                            last_num = n;
                            continue;
                        }
                        Err(_e) => {}
                    }

                    for i in 0..colors.len() {
                        match part.find(colors[i]) {
                            Some(_x) => {
                                colors_numbers[i] += last_num;
                            }
                            None => {}
                        }
                    }
                }
            }

            for (x, y) in colors_numbers.iter_mut().zip(minimum_color.iter_mut()) {
                if x > y {
                    *y = *x;
                }
            }
        }

        for min in minimum_color {
            power *= min;
        }

        sum += power;
    }

    println!("{}", sum);
}
