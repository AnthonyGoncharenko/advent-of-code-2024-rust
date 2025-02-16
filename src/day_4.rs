fn is_xmas(s: &str) -> bool {
    return s == "XMAS" || s == "SMAX";
}

fn count_xmases(file: &str, line_idx: usize, char_idx: usize) -> i32 {
    let mut num_xmases: i32 = 0;
    let line: &str = file.lines().nth(line_idx).unwrap();

    // Forward Horizontal Check
    if let Some(chars) = line.get(char_idx..char_idx+4) {
        if is_xmas(&chars) {
            num_xmases += 1;
        }
    }

    // Backward Horizontal Check
    if char_idx as i32 - 3 >= 0 {
        if let Some(chars) = line.get(char_idx-3..=char_idx).and_then(|s| Some(s.chars().rev().collect::<String>())) {
            if is_xmas(&chars) {
                num_xmases += 1;
            }
        }
    }

    // Forward Vertical Check 
    let mut chars: String = String::new();
    for line in file.lines().skip(line_idx).take(4) {
        if let Some(char) = line.get(char_idx..char_idx+1) {
            chars.push_str(char);
        }
    }

    if is_xmas(&chars) {
        num_xmases += 1;
    }

    // Backward Vertical Check 
    let mut chars: String = String::new();
    if line_idx as i32 - 3 >= 0 {
        for line in file.lines().skip(line_idx-3).take(4) {
            if let Some(char) = line.get(char_idx..char_idx+1) {
                chars.push_str(char);
            }
        }

        chars = chars.chars().rev().collect();

        if is_xmas(&chars) {
            num_xmases += 1;
        }
    }

    // South East Check
    let mut chars: String = String::new();
    for (offset, line) in file.lines().skip(line_idx).take(4).enumerate() {
        if let Some(char) = line.get(char_idx+offset..char_idx+offset+1) {
            chars.push_str(char);
        }
    }

    if is_xmas(&chars) {
        num_xmases += 1;
    }

    // South West Check
    let mut chars: String = String::new();
    if char_idx as i32 - 3 >= 0 {
        for (offset, line) in file.lines().skip(line_idx).take(4).enumerate() {
            if let Some(char) = line.get(char_idx-offset..char_idx-offset+1) {
                chars.push_str(char);
            }
        }
    
        if is_xmas(&chars) {
            num_xmases += 1;
        }
    }

    // North East Check
    if line_idx as i32 - 3 >= 0 {
        let mut chars: String = String::new();
        for (offset, line) in file.lines().skip(line_idx-3).take(4).enumerate() {
            if let Some(char) = line.get(char_idx+3-offset..char_idx+3-offset+1) {
                chars.push_str(char);
            }
        }

        chars = chars.chars().rev().collect();

        if is_xmas(&chars) {
            num_xmases += 1;
        }
    }

    // North West Check
    if line_idx as i32 - 3 >= 0 && char_idx as i32 - 3 >= 0 {
        let mut chars: String = String::new();
        for (offset, line) in file.lines().skip(line_idx-3).take(4).enumerate() {
            if let Some(char) = line.get(char_idx-3+offset..char_idx-3+offset+1) {
                chars.push_str(char);
            }
        }

        chars = chars.chars().rev().collect();
    
        if is_xmas(&chars) {
            num_xmases += 1;
        }
    }

    num_xmases
}

fn is_x_mas(char_matrix: &[[char;3];3]) -> bool {
    if char_matrix[0][0] == 'M' && char_matrix[0][2] == 'M' {
        return char_matrix[0][0] == 'M' && char_matrix[1][1] == 'A' && char_matrix[2][2] == 'S' &&
               char_matrix[0][2] == 'M' && char_matrix[1][1] == 'A' && char_matrix[2][0] == 'S';
    } else if char_matrix[0][0] == 'M' && char_matrix[2][0] == 'M' {
        return char_matrix[0][0] == 'M' && char_matrix[1][1] == 'A' && char_matrix[2][2] == 'S' &&
               char_matrix[2][0] == 'M' && char_matrix[1][1] == 'A' && char_matrix[0][2] == 'S';
    } else if char_matrix[0][2] == 'M' && char_matrix[2][2] == 'M' {
        return char_matrix[0][2] == 'M' && char_matrix[1][1] == 'A' && char_matrix[2][0] == 'S' &&
               char_matrix[2][2] == 'M' && char_matrix[1][1] == 'A' && char_matrix[0][0] == 'S';
    } else if char_matrix[2][0] == 'M' && char_matrix[2][2] == 'M' {
        return char_matrix[2][0] == 'M' && char_matrix[1][1] == 'A' && char_matrix[0][2] == 'S' &&
               char_matrix[2][2] == 'M' && char_matrix[1][1] == 'A' && char_matrix[0][0] == 'S';
    } else {
        return false;
    }
}

pub fn day_4() {
    let file: &str = include_str!("inputs/day_4.txt");
    let mut result_1: i32 = 0;
    
    for (line_idx, line) in file.lines().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c == 'X' {
                result_1 += count_xmases(&file, line_idx, char_idx);
            }
        }
    }
    println!("Result 1: {result_1}");
    
    let mut result_2: i32 = 0;
    let lines_vec: Vec<&str> = file.lines().collect::<Vec<_>>();
    let three_lines: std::slice::Windows<'_, &str> = lines_vec.windows(3);
    let line_len: usize = file.lines().take(1).collect::<String>().len();

    for three_line in three_lines {
        //   [.............]
        //   [.............]
        //   [.............]
        for offset in 0..line_len {
            //   [.............]
            //         ^
            let mut window: [[char; 3]; 3] = [['0', '0', '0'], ['0', '0', '0'], ['0', '0', '0']];
            for (line_idx ,line ) in three_line.iter().enumerate() {
                //   [.............]
                for (c_idx, c) in line.chars().skip(offset).take(3).enumerate() {
                    //   [.............]
                    //         ^--^
                    window[line_idx][c_idx] = c;
                }
            }
            if is_x_mas(&window) {
                result_2 += 1;
            }
        }
    }
    println!("Result 2: {result_2}");
}
