use std::fs::File;
use std::io::{self, BufRead};

fn is_safe_row(row: &[i32]) -> bool {
    let mut prev = row[0];
    for &level in &row[1..] {
        let diff = (prev - level).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        prev = level;
    }
    let is_increasing = row.windows(2).all(|pair| pair[0] < pair[1]);
    let is_decreasing = row.windows(2).all(|pair| pair[0] > pair[1]);

    is_increasing || is_decreasing
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "assets/lists.txt";
    let mut safe_part1 = 0;
    let mut safe_part2 = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        matrix.push(row);
    }

    for row in &matrix {
        if is_safe_row(row) {
            safe_part1 += 1;
        } else {
            for i in 0..row.len() {
                let mut temp_row = row.clone();
                temp_row.remove(i);

                if is_safe_row(&temp_row) {
                    safe_part2 += 1;
                    break;
                }
            }
        }
    }

    println!("Part1: {}", safe_part1);
    println!("Part2: {}", safe_part1 + safe_part2);
    Ok(())
}
