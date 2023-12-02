fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    
    //Part1
    let total: u32 = input.lines()
                          .map(|line| {
                            let mut num: u32 = 0;
                            for ch in line.chars() {
                                if ch.is_digit(10) {
                                    num += ch.to_digit(10).unwrap() * 10;
                                    break;
                                }
                            }
                            for ch in line.chars().rev() {
                                if ch.is_digit(10) {
                                    num += ch.to_digit(10).unwrap();
                                    break;
                                }
                            }
                            num
                          })
                          .sum();
    
    println!("Part 1: {}", total);

    //Part2
    let digits = vec![("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
                      ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let total2: u32 = input.lines()
                        .map(|line| {
                            let mut digits_found: Vec<u32> = Vec::new();
                            let mut line = line.chars().peekable();

                            while let Some(ch) = line.next() {
                                if ch.is_digit(10) {
                                    digits_found.push(ch.to_digit(10).unwrap() as u32);
                                    continue;
                                }
                                let next_4: String = std::iter::once(ch).chain(line.clone().take(4)).collect();

                                for (d, val) in &digits {
                                    if next_4.starts_with(d) {
                                        digits_found.push(*val);
                                    }
                                }
                            }
                            let first: u32 = *digits_found.first().unwrap();
                            let last: u32 = *digits_found.last().unwrap();
                            first * 10 + last
                        })
                        .sum();

    println!("Part 2: {}", total2);

}
