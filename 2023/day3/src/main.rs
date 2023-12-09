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
}
