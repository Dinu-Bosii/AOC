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
}


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
        _ => 1
    } 
}

// a,x - rock(+2), b,y - paper(+2), c,z - scissors(+3)
