fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let chars: Vec<char> = input
        .chars()
        .filter(|&ch| !ch.is_whitespace())
        .collect();

    let mut letters = [false; 26]; 
    let position: Option<u32> = chars
        .windows(4)
        .enumerate()
        .find_map(|(index, chars)| {
            let mut ret: bool = true;
            for &ch in chars {
                let ch_index = ch as usize - 'a' as usize;
                if letters[ch_index] == false {
                    letters[ch_index] = true;
                }
                else {
                    ret = false;
                }
            }
            if ret {
                println!("chars >> {:?}", chars);
                return Some((index + 4) as u32);
            }
            else {

                for &ch in chars {
                    let ch_index = ch as usize - 'a' as usize;
                    letters[ch_index] = false;
                }
                None
            } 
        })
        ;
    println!("Number of chars before the marker: {}\n", position.unwrap());

    //Part2
    let mut letters2 = [false; 26]; 
    let position2: Option<u32> = chars
        .windows(14)
        .enumerate()
        .find_map(|(index, chars)| {
            let mut ret: bool = true;
            for &ch in chars {
                let ch_index = ch as usize - 'a' as usize;
                if letters2[ch_index] == false {
                    letters2[ch_index] = true;
                }
                else {
                    ret = false;
                }
            }
            if ret {
                println!("chars >> {:?}", chars);
                return Some((index + 14) as u32);
            }
            else {

                for &ch in chars {
                    let ch_index = ch as usize - 'a' as usize;
                    letters2[ch_index] = false;
                }
                None
            } 
        })
        ;
    println!("Number of chars before the start-of-message marker: {}", position2.unwrap());

}
