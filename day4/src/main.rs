fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let total_overlap: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let assignments: Vec<&str> = line.split(',').collect();
            let section1: Vec<&str> = assignments[0].split('-').collect();
            let section2: Vec<&str> = assignments[1].split('-').collect();
            let s1 = section1[0].parse::<u32>().unwrap();
            let e1 = section1[1].parse::<u32>().unwrap();
            let s2 = section2[0].parse::<u32>().unwrap();
            let e2 = section2[1].parse::<u32>().unwrap();
            if s1 <= s2 {
                if e1 >= e2 {
                    return 1;
                }
            }
            if s1 >= s2 {
                if e1 <= e2 {
                    return 1;
                }
            }
            0
            
        })
        .sum();
        
    println!("Total overlapping sections: {}", total_overlap);    
}


