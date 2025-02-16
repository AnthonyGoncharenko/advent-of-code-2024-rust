use regex::Regex;

pub fn day_3() {
    let file = include_str!("inputs/day_3.txt");

    let mut part_1 = 0;
    let mut part_2 = 0;
    let re: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut is_do: bool = true;
    for capture in re.captures_iter(file){
        let instruction = capture.get(0).unwrap().as_str();
        
        if "mul" == &instruction[0..3] {
            let num_1: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let num_2: i32 = capture.get(2).unwrap().as_str().parse().unwrap();

            part_1 +=  num_1 * num_2;

            if is_do {
                part_2 +=  num_1 * num_2;
            }
        } else if "do(" == &instruction[0..3] {
            is_do = true;
        } else if "don" == &instruction[0..3] {
            is_do = false;
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}