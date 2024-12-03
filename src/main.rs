use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "assets/lists.txt";
    let mut safe = 0;

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
        let mut is_safe = true;
        let mut prev = row[0];

        for &level in &row[1..] {
            let diff = (prev - level).abs();
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
            prev = level;
        }

        if is_safe {
            let is_increasing = row.windows(2).all(|pair| pair[0] < pair[1]);
            let is_decreasing = row.windows(2).all(|pair| pair[0] > pair[1]);

            if is_increasing || is_decreasing {
                safe += 1;
            }
        }
    }

    println!("Part1: {}", safe);
    Ok(())
}
