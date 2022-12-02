use std::vec;

use crate::get_inputs;

pub fn run() {
    let input = get_inputs(2022, 1);
    let mut count = 0;
    let mut calories: vec::Vec<i32> = vec::Vec::new();
    for line in input {
        if line == "" {
            calories.push(count);
            count = 0;
            continue;
        }
        println!("{line}");
        let c: i32 = line.parse().expect("unable to parse i32 from line");
        count += c;
    }
    calories.sort_by(|a, b| b.cmp(a));

    println!(
        "day 1 part 1: {:?}",
        calories.first().expect("unable to get first item in vec")
    );
    let top_3: i32 = calories[0..3].iter().sum();

    println!("day 1 part 2: {:?}", top_3);
}
