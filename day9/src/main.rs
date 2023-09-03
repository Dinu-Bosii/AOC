use std::collections::HashSet;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    
    let mut positions: HashSet<(i16, i16)> = HashSet::new();
    positions.insert((0, 0));
    let mut head: (i16, i16) = (0 , 0);
    let mut tail: (i16, i16) = (0 , 0);

    let _ = input
        .lines()
        .for_each(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let letter = line_parts[0];
            let number = line_parts[1].parse::<i16>().unwrap(); 
            match letter {
                "L" => {
                    for _ in 1..=number {
                        head.1 -= 1;
                        if tail.1 > head.1 + 1 {
                            tail.0 = head.0;
                            tail.1 = head.1 + 1;
                            positions.insert(tail);
                        }
                    }
                }
                "R" => {                    
                    for _ in 1..=number {
                        head.1 += 1;
                        if tail.1 < head.1 - 1 {
                            tail.0 = head.0;
                            tail.1 = head.1 - 1;
                            positions.insert(tail);
                        }
                    }
                }
                "U" => {                    
                    for _ in 1..=number {
                        head.0 += 1;
                        if tail.0 < head.0 - 1 {
                            tail.0 = head.0 - 1;
                            tail.1 = head.1;
                            positions.insert(tail);
                        }
                    }
                }
                "D" => {
                    for _ in 1..=number {
                        head.0 -= 1;
                        if tail.0 > head.0 + 1 {
                            tail.0 = head.0 + 1;
                            tail.1 = head.1;
                            positions.insert(tail);
                        }
                    }
                }
                _ => {
                    println!("why am i here");
                }
            }
        });    
    println!("The tail visited {} positions", positions.len());
}
