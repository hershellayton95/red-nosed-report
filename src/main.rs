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
        let diff: Vec<i32> = row.windows(2)
            .map(|pair| pair[0] - pair[1])
            .collect();

        let abs_diff_sum: i32 = diff.windows(2)
            .map(|pair| pair[0].abs() + pair[1].abs())
            .sum();

        let diff_sum: i32 = diff.windows(2)
            .map(|pair| pair[0] + pair[1])
            .sum::<i32>();

        if abs_diff_sum == diff_sum {
            let correct2 = diff.iter().all(|&num| num.abs() <= 3);

            if correct2 {
                safe += 1;
            }
        }
    }

    println!("Part1: {}", safe);
    Ok(())
}
