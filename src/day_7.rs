use std::{collections::VecDeque, ops::{Add, Div, Mul, Sub}};

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

#[derive(Debug, Clone, Copy)]
struct Calculation {
    lhs: i128,
    rhs: i128,
    operation: Op,
}

fn validate_line_new_method(calibration: i128, list: &[i128]) -> bool {
    let mut open = VecDeque::new();
    
    let idx = 0;
    let lhs = *list.get(idx).unwrap();
    let rhs = *list.get(idx+1).unwrap();

    open.push_back((Calculation{
            lhs,
            rhs,
            operation: Op::Mul,
        },
    idx+1));
    open.push_back((Calculation{
            lhs,
            rhs,
            operation: Op::Add,
        },
    idx+1));
    open.push_back((Calculation{
            lhs,
            rhs,
            operation: Op::Concat,
        },
    idx+1));

    while open.len() > 0 {
        let (calculation, idx) = open.pop_front().unwrap();
        if idx >= list.len() -1  {

            match calculation.operation {
                Op::Mul => {
                    if calculation.lhs.mul(calculation.rhs) == calibration {
                        return true;
                    }
                },
                Op::Add => {
                    if calculation.lhs.add(calculation.rhs) == calibration {
                        return true;
                    }
                }
                Op::Concat => {
                    let lhs = calculation.lhs.to_string();
                    let rhs = calculation.rhs.to_string();
                    if (lhs+&rhs).parse::<i128>().unwrap() == calibration {
                        return true;
                    }
                },
            }
        } else {

            let new_lhs: i128 = match calculation.operation {
                Op::Mul => calculation.lhs.mul(calculation.rhs),
                Op::Add => calculation.lhs.add(calculation.rhs),
                Op::Concat => {
                    let lhs = calculation.lhs.to_string();
                    let rhs = calculation.rhs.to_string();
                    (lhs + &rhs).parse().unwrap()
                },
            };
            let new_idx = idx + 1;
            if new_idx < list.len() {
                let new_rhs = *list.get(new_idx).unwrap();
        
                open.push_front((Calculation{
                    lhs: new_lhs,
                    rhs: new_rhs,
                    operation: Op::Mul,
                }, new_idx));
        
                open.push_front((Calculation{
                    lhs: new_lhs,
                    rhs: new_rhs,
                    operation: Op::Add,
                }, new_idx));
        
                open.push_front((Calculation{
                    lhs: new_lhs,
                    rhs: new_rhs,
                    operation: Op::Concat,
                }, new_idx));
            }
        }
    }

    return false;
}

#[derive(Debug, Clone, Copy)]
enum InverseOp {
    Div,
    Sub
}

#[derive(Debug, Clone, Copy)]
struct InverseCalculation {
    lhs: i128,
    rhs: i128,
    operation: InverseOp,
}

fn add_new_inverse_calculation(open: &mut VecDeque<(InverseCalculation, usize)>, lhs: i128, rhs: i128 ,new_idx: usize) {
    if lhs % rhs == 0 {
        open.push_back((InverseCalculation{
            lhs,
            rhs,
            operation: InverseOp::Div,
        }, new_idx));
    }

    if lhs - rhs >= 0 {
        open.push_back((InverseCalculation{
            lhs,
            rhs,
            operation: InverseOp::Sub,
        }, new_idx));
    } 
}

fn validate_line_inverse_method(calibration: i128, list: &[i128]) -> bool {
    let mut open: VecDeque<(InverseCalculation, usize)> = VecDeque::new();

    let curr_num: i128 = *list.get(0).unwrap();

    if calibration % curr_num == 0 {
        open.push_back((InverseCalculation{
            lhs: calibration,
            rhs: curr_num,
            operation: InverseOp::Div,
        }, 0));
    }

    if calibration - curr_num >= 0 {
        open.push_back((InverseCalculation{
            lhs: calibration,
            rhs: curr_num,
            operation: InverseOp::Sub,
        }, 0));
    }
    while open.len() > 0 {
        let (calculation, idx) = open.pop_front().unwrap();
        let new_idx = idx + 1;

        if new_idx >= list.len() {
            match calculation.operation {
                InverseOp::Div => {
                    if calculation.lhs.div(calculation.rhs) == 1 {
                        return true;
                    }
                },
                InverseOp::Sub => {
                    if calculation.lhs.sub(calculation.rhs) == 0 {
                        return true;
                    }
                }
            }
        } else {
            match calculation.operation {
                InverseOp::Div => {
                    let new_calibration = calculation.lhs.div(calculation.rhs);
                    if let Some(&rhs) = list.get(new_idx) {
                        add_new_inverse_calculation(&mut open, new_calibration, rhs, new_idx);
                    }
                },
                InverseOp::Sub => {
                    let new_calibration = calculation.lhs.sub(calculation.rhs);
                    if let Some(&rhs) = list.get(new_idx) {
                        add_new_inverse_calculation(&mut open, new_calibration, rhs, new_idx);
                    }
                },
            }
        }
    }

    return false;
}


pub fn day_7(){
    let file: &str= include_str!("inputs/day_7.txt");
    let mut result_1: i128 = 0;
    let mut result_2: i128 = 0;

    for line in file.lines() {
        if let Some((calibration, list)) = line.split_once(":") {
            let calibration: i128 = calibration.parse().unwrap();

            let num_list: Vec<i128> = list
                                        .split_ascii_whitespace()
                                        .map(|s| s.parse::<i128>().unwrap())
                                        .rev()
                                        .collect();
            if validate_line_inverse_method(calibration, &num_list) {
                result_1 += calibration;
                result_2 += calibration;
            } else {
                let num_list: Vec<i128> = num_list.clone().into_iter().rev().collect();
                if validate_line_new_method(calibration, &num_list) {
                    result_2 += calibration;
                }
            }
        }
    }

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}