fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let mut cycle: i32 = 0;
    let mut sum_signal_strength: i32 = 0;
    let mut x: i32 = 1;
    
    //Part1
    let _ = input
        .lines()
        .for_each(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let op = line_parts[0];
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                sum_signal_strength += x * cycle;
            }

            if op == "addx" {
                let number = line_parts[1].parse::<i32>().unwrap();
                cycle += 1;
                if (cycle + 20) % 40 == 0 {
                    sum_signal_strength += x * cycle;
                }
                x += number;
            }
        });

    println!("Sum of signal strengths: {}", sum_signal_strength);

    //Part2
    let _ = input
}
