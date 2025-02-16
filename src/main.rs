use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut fns: [Option<fn()>; 25] = [None; 25];
    fns[0] = Some(day_1::day_1);
    fns[1] = Some(day_2::day_2);
    fns[2] = Some(day_3::day_3);
    fns[3] = Some(day_4::day_4);
    fns[4] = Some(day_5::day_5);
    fns[5] = Some(day_6::day_6);
    fns[6] = Some(day_7::day_7);
    if args.len() > 0 {
        for arg in args {
            if let Ok(num) = arg.parse::<i32>() {
                if num >= 1 && num <= 25 {
                    if let Some(aoc_fn) = fns[num as usize - 1] {
                        println!("Day {}", num);
                        aoc_fn();
                        println!("------------------");
                    }
                }
            }
        }
    } else {
        for (fn_idx, aoc_fn) in fns.iter().enumerate() {
            if let Some(aoc_fn) = aoc_fn {
                println!("Day {}", fn_idx+1);
                aoc_fn();
                println!("------------------");
            }
        }

    }
}