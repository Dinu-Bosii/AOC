fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();

    let part1_lines: Vec<&str> = parts[0].lines().collect();

    let num_stacks: Vec<u16> = part1_lines[part1_lines.len() - 1]
        .split_whitespace()
        .map(|s| s.parse::<u16>().unwrap_or(0))
        .collect();

    let total_stacks: u16 = num_stacks[num_stacks.len() - 1];
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_stacks {
        stacks.push(Vec::new());
    }
    for i in (0..part1_lines.len() - 1).rev() {
        let mut j: usize = 0;
        for (index, c) in part1_lines[i].char_indices() {
            if index > 0 && index % 4 == 1 {
                if c.is_alphabetic() {
                    stacks[j].push(c); 
                }
                j += 1;
            }  
        }    
    }

    parts[1].lines().for_each(|line| {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let amount: u32 = line_parts[1].parse::<u32>().unwrap();
        let from: usize = line_parts[3].parse::<usize>().unwrap();
        let to: usize = line_parts[5].parse::<usize>().unwrap();
        for _ in 0..amount {
            let ch: char = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(ch);
        }
    });

    println!("Crates on top: {}", stacks
        .iter()
        .map(|row| row.last())
        .flatten()  
        .collect::<String>()
    );
     
    //Part2
    let mut stacks2: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_stacks {
        stacks2.push(Vec::new());
    }
    for i in (0..part1_lines.len() - 1).rev() {
        let mut j: usize = 0;
        for (index, c) in part1_lines[i].char_indices() {
            if index % 4 == 1 {
                if c.is_alphabetic() {
                    stacks2[j].push(c); 
                }
                j += 1;
            }  
        }    
    }
    parts[1].lines().for_each(|line| {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let amount: u32 = line_parts[1].parse().unwrap();
        let from: usize = line_parts[3].parse().unwrap();
        let to: usize = line_parts[5].parse().unwrap();
        for i in (0..amount).rev() {
            let index : usize = stacks2[from - 1].len() - i as usize - 1;
            let ch: char = stacks2[from - 1].remove(index);
            stacks2[to - 1].push(ch);
        }
    });

    println!("Crates on top part 2: {}", stacks2
        .iter()
        .map(|row| row.last())
        .flatten()  
        .collect::<String>()
    );

}
    
