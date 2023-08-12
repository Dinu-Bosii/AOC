use std::char;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let total_priority: u32 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(half1, half2)| {
            let mut unique_ch: Vec<char> = Vec::new();
            let mut sum: u32 = 0;

            for ch in half1.chars() {
                if half2.contains(ch) && !unique_ch.contains(&ch) {
                    unique_ch.push(ch);
                    if ch.is_ascii_lowercase() {
                        sum += ch as u32 - 'a' as u32 + 1;
                    } else {
                        sum += ch as u32 - 'A' as u32 + 27;
                    }
                }
            }
            sum
        })
        .sum();
        
    println!("Sum of the priorities: {}", total_priority);
    
    let total_group_priority: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            for ch in chunk[0].chars() {
                if chunk[1].contains(ch) && chunk[2].contains(ch) {
                    if ch.is_ascii_lowercase() {
                        return ch as u32 - 'a' as u32 + 1;
                    } else {
                        return ch as u32 - 'A' as u32 + 27;
                    }
                }
            }
            0
        })
        .sum();

    println!("Sum of the group priorities: {}", total_group_priority);    
}


