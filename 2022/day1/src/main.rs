use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt");
    let input = BufReader::new(file.unwrap());
    
    let mut best: u32 = 0;
    let mut curr: u32 = 0;
    let mut best2: u32 = 0;
    let mut best3: u32 = 0;
    for line in input.lines() {
        if let Ok(line) = line {
            if let Ok(number) = line.trim().parse::<u32>() {
                curr += number;    
            }
            else if line.trim().is_empty() {
                if curr > best {
                    best3 = best2;
                    best2 = best;
                    best = curr;
                }
                curr = 0;
            }

        }
    }
    println!("The best is {best}");
    println!("Top 3 is {}", best + best2 + best3);


}

