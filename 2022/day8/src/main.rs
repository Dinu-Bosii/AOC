fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let trees: Vec<Vec<u8>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut digits: Vec<u8> = Vec::new();
            for ch in line.chars() {
                digits.push(ch.to_digit(10).unwrap() as u8);
            }
            return digits;
           })
        .collect();

    let rows: usize = trees.len();
    let cols: usize = trees[0].len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    
    //Part1 - Strictly increasing sequences
    //Left to Right and Right to Left
    for i in 1..rows - 1 {
        let mut curr_height = trees[i][0];
        for j in 1..cols - 1 {
            if trees[i][j] > curr_height {
                curr_height = trees[i][j];
                visible[i][j] = true;         
            }
        }
        curr_height = trees[i][cols - 1];
        for j in (1..cols - 1).rev() {
            if trees[i][j] > curr_height {
                curr_height = trees[i][j];
                visible[i][j] = true;         
            }
        }


    } 
    //Top to Bottom and Bottom to Top 
    for j in 1..cols - 1 {
        let mut curr_height = trees[0][j];
        for i in 1..rows - 1 {
            if trees[i][j] > curr_height {
                curr_height = trees[i][j];
                visible[i][j] = true;         
            }
        }

        curr_height = trees[rows - 1][j];
        for i in (1..rows - 1).rev() {
            if trees[i][j] > curr_height {
                curr_height = trees[i][j];
                visible[i][j] = true;         
            }
        }
    }
    let mut visible_trees = visible
        .iter()
        .skip(1) 
        .take(rows - 2) 
        .flat_map(|row| row.iter().skip(1).take(cols - 2)) 
        .filter(|&&visible| visible)
        .count();
    visible_trees += rows * 2 + (cols - 2) * 2;
    println!("Number of visible trees: {}", visible_trees);
    
    //Part2 
    let mut best: u32 = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {

            let mut left_score: u32 = 0;
            for left in (0..j).rev() {
                    left_score += 1;
                if trees[i][j] <= trees[i][left] {
                    break;
                }
            }

            let mut right_score: u32 = 0;
            for right in j+1..cols {
                right_score += 1;
                if trees[i][j] <= trees[i][right] {
                    break;
                }
            }

            let mut up_score: u32 = 0;
            for up in (0..i).rev() {
                up_score += 1;
                if trees[i][j] <= trees[up][j] {
                    break;
                }
            }

            let mut down_score: u32 = 0;
            for down in i+1..rows {
                down_score += 1;
                if trees[i][j] <= trees[down][j] {
                    break;
                }
            }

            let scenic_score: u32 = down_score * right_score * up_score * left_score;
            if scenic_score > best {
                best = scenic_score;
            }
        }
    }
    println!("Best scenic score: {}",  best);

}
