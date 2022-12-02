use crate::get_inputs;
use std::collections::HashMap;

pub fn run() {
    let input = get_inputs(2022, 2);
    
    let mut scores = HashMap::<String, (i32, i32)>::new();

    scores.insert(String::from("A X"), (4, 3));
    scores.insert("A Y".into(), (8, 4));
    scores.insert("A Z".into(), (3, 8));
    
    scores.insert(String::from("B X"), (1, 1));
    scores.insert("B Y".into(), (5, 5));
    scores.insert("B Z".into(), (9, 9));

    scores.insert(String::from("C X"), (7, 2));
    scores.insert("C Y".into(), (2, 6));
    scores.insert("C Z".into(), (6, 7));

    let mut part1 = 0;
    let mut part2 = 0;

    for line in input {
        let (a, b) = scores.get(&line).unwrap_or(&(0,0));
        part1 += a;
        part2 += b;
    }
    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
