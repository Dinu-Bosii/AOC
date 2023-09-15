use std::collections::HashSet;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    
    let mut positions: HashSet<(i16, i16)> = HashSet::new();
    positions.insert((0, 0));
    let mut head: (i16, i16) = (0 , 0);
    let mut tail: (i16, i16) = (0 , 0);
    //Part1
    let _ = input
        .lines()
        .for_each(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let letter = line_parts[0];
            let number = line_parts[1].parse::<i16>().unwrap();
            //Tail goes to Head's last position(doesn't work for part 2)
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
                    unreachable!();
                }
            }
        });    
    println!("The tail visited {} positions", positions.len());

    //Part2
    positions.clear();
    positions.insert((0, 0));
    let mut knots: Vec<(i16, i16)> = vec![(0 , 0); 10];

    let _ = input
        .lines()
        .for_each(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let letter = line_parts[0];
            let number = line_parts[1].parse::<i16>().unwrap();
            for _ in 1..=number {
                match letter {
                    "L" => knots[0].1 -= 1,
                    "R" => knots[0].1 += 1,
                    "U" => knots[0].0 += 1,
                    "D" => knots[0].0 -= 1,
                    _ => unreachable!(),
                }
                for i in 1..10 {  
                    //Tail moves cardinally if its on the same 'cardinal' path 
                    //Otherwise it moves in a diagonal in the Head's direction
                    //For a better understanding:
                    //https://galaxyinferno.com/how-to-solve-advent-of-code-2022-day-9-with-python/
                    //Could be simplified to a single 'if'
                    let dist = (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);  
                    if dist.0 > 1 {
                        knots[i].0 += 1;
                        if dist.1 > 0 {
                            knots[i].1 += 1;
                        }
                        else if dist.1 < 0 {
                            knots[i].1 -= 1;
                        }
                    }
                    else if dist.0 < -1 {
                        knots[i].0 -= 1;
                        if dist.1 > 0 {
                            knots[i].1 += 1;
                        }
                        else if dist.1 < 0 {
                            knots[i].1 -= 1;
                        }
                    }
                    else if dist.1 > 1 {                                   
                        knots[i].1 += 1;
                        if dist.0 > 0 {
                            knots[i].0 += 1;
                        }
                        else if dist.0 < 0 {
                            knots[i].0 -= 1;
                        }
                    }
                    else if dist.1 < -1 {                                   
                        knots[i].1 -= 1;
                        if dist.0 > 0 {
                            knots[i].0 += 1;
                        }
                        else if dist.0 < 0 {
                            knots[i].0 -= 1;
                        }
                    }
                }
                positions.insert(knots[9]);
            }                     
        });


    println!("The tail 9 visited {} positions", positions.len());

}
