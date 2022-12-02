use crate::get_inputs;
use std::collections::HashMap;

pub fn run() {
    let input = get_inputs(2022, 2);
    
    let mut scores = HashMap::new();

    scores.insert("A X".to_string(), (4, 3));
    scores.insert("A Y".to_string(), (8, 4));
    scores.insert("A Z".to_string(), (3, 8));
    
    scores.insert("B X".to_string(), (1, 1));
    scores.insert("B Y".to_string(), (5, 5));
    scores.insert("B Z".to_string(), (9, 9));

    scores.insert("C X".to_string(), (7, 2));
    scores.insert("C Y".to_string(), (2, 6));
    scores.insert("C Z".to_string(), (6, 7));

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
