use std::char;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let score: u32 = input.chars()
                          .filter(|c| !c.is_whitespace())
                          .collect::<Vec<char>>() 
                          .chunks(2) 
                          .map(|letters| get_score((letters[0], letters[1])))
                          .sum();
    println!("Total score: {}", score);

    let score2: u32 = input.chars()
                          .filter(|c| !c.is_whitespace())
                          .collect::<Vec<char>>() 
                          .chunks(2) 
                          .map(|letters| get_score2((letters[0], letters[1])))
                          .sum();
    println!("Total score2: {}", score2);
}

//Part1
fn get_score(letters: (char, char)) -> u32 {
    match letters {
        ('A', 'Y') => 6 + 2,
        ('A', 'X') => 3 + 1,
        ('B', 'Z') => 6 + 3,
        ('B', 'Y') => 3 + 2,
        ('C', 'X') => 6 + 1,
        ('C', 'Z') => 3 + 3,
        (_, 'X') => 1,
        (_, 'Y') => 2,
        (_, 'Z') => 3,
        _ => 0
    } 
}

//Part2 
fn get_score2(letters: (char, char)) -> u32 {
    match letters {
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('A', 'X') => 0 + 3,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('B', 'X') => 0 + 1,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        ('C', 'X') => 0 + 2,
        _ => 0
    } 
}

