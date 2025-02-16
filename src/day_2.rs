

fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut increasing: bool = true;

    for col_idx in 1..report.len() {
        let curr = report.get(col_idx).unwrap();
        let prev = report.get(col_idx-1).unwrap();
    
        let diff = curr-prev;
        let diff_magnitude = diff.abs();
        if !(1 <= diff_magnitude && diff_magnitude <= 3) {
            return false;
        }
        if col_idx == 1 {
            increasing = diff > 0;
        } else {
            if (diff > 0) != increasing {
                return false;
            }
        }
    }

    true
}

pub fn day_2() {
    let file = include_str!("inputs/day_2.txt");

    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in file.lines() {
        let report = line.split_ascii_whitespace().into_iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        if is_safe_report(&report) {
            part_1 += 1;
            part_2 += 1;
        } else {
            for test_remove_idx in 0..report.len() {
                let mut test_report = report.clone();
                test_report.remove(test_remove_idx);
                if is_safe_report(&test_report) {
                    part_2 += 1;
                    break;
                }
            }
        }
    }
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}