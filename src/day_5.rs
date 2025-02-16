use std::{cmp::Ordering, collections::HashMap, ops::Div};

fn is_sorted(nums: &Vec<i32>, adj_list: & HashMap<(i32, i32), Ordering>) -> bool {
    for window in nums.windows(2) {
        let x: &i32 = window.get(0).unwrap();
        let y: &i32 = window.get(1).unwrap();
        
        if !adj_list.get(&(*x, *y)).unwrap().is_lt() {
            return false;
        }
    }
    return true;
}

pub fn day_5() {
    let file: &str = include_str!("inputs/day_5.txt");
    let mut split_file: std::str::SplitTerminator<'_, &str> = file.split_terminator("\r\n\r\n");

    let mut result_1: i32 = 0;
    let mut result_2: i32 = 0;

    let rules: &str = split_file.next().unwrap();
    let mut adj_list: HashMap<(i32, i32), Ordering>= std::collections::HashMap::new(); 
    for rule in rules.lines() {
        let (x, y)  = rule.split_once("|").unwrap();
        let x: i32 = x.parse::<i32>().unwrap();
        let y: i32 = y.parse::<i32>().unwrap();

        adj_list.insert((x, y), Ordering::Less);
        adj_list.insert((y, x), Ordering::Greater);
        adj_list.insert((x, x), Ordering::Equal);
        adj_list.insert((y, y), Ordering::Equal);
    }
    
    let orders: &str = split_file.next().unwrap();
    for order in orders.lines() {
        if let Ok(nums) = order.split(",").map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>() {
            let mut ok: bool = true;
            for window in nums.windows(2) {
                let x: &i32 = window.get(0).unwrap();
                let y: &i32 = window.get(1).unwrap();
                
                if !adj_list.get(&(*x, *y)).unwrap().is_lt() {
                    ok = false;
                    break;
                }
            }

            if ok {
                result_1 += nums.get(nums.len().div(2) as usize).unwrap();
            } else {
                // Correctly Order Nums
                let mut nums = nums.clone();
                while !is_sorted(&nums, &adj_list) {
                    for num_idx in 1..nums.len() {
                        let x: &i32 = nums.get(num_idx-1).unwrap();
                        let y: &i32 = nums.get(num_idx).unwrap();
                        
                        if !adj_list.get(&(*x, *y)).unwrap().is_lt() {
                            nums.swap(num_idx, num_idx-1);
                        }
                    }
                }
                result_2 += nums.get(nums.len().div(2) as usize).unwrap();
            }
        }
    }
    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}