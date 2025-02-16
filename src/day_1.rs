use std::collections::HashMap;

pub fn day_1() {
    let file: &str= include_str!("inputs/day_1.txt");
    let mut col_1: Vec<i32> = Vec::new();
    let mut col_2: Vec<i32> = Vec::new();
    
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for line in file.lines() {
        let mut split: std::str::SplitAsciiWhitespace<'_> = line.split_ascii_whitespace();
        let x: i32 = split.next().unwrap().parse::<i32>().unwrap();
        col_1.push(x);
        let y: i32 = split.next().unwrap().parse::<i32>().unwrap();
        col_2.push(y);
        if !counter.contains_key(&y) {
            counter.insert(y, 1);
        } else {
            let v: &mut i32 = counter.get_mut(&y).unwrap();
            *v += 1;
        }
    }
    col_1.sort();
    col_2.sort();

    let mut result_1: i32 = 0;
    let mut result_2: i32 = 0;
    for (x, y) in col_1.iter().zip(col_2) {
        result_1 += (x-y).abs();
        result_2 += x * counter.get(x).unwrap_or(&0);
    }
    println!("Result 1: {result_1}");
    println!("Result 2: {result_2}");
}