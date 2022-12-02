use std::fs::File;
use std::io::Read;
use std::vec::Vec;

fn get_input() -> Vec<i32> {
    let mut puzzle_input = String::new();

    File::open("aoc_2021/day1.txt")
        .expect("could not read file")
        .read_to_string(&mut puzzle_input)
        .expect("error reading file");

    let input_lines = puzzle_input.split("\n");

    let mut measurements: Vec<i32> = Vec::new();
    for number in input_lines {
        let number = match number.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        measurements.push(number);
    }
    return measurements;
}

pub fn run() {
    let input = get_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i32>) {
    let mut p1_count = 0;
    let mut p1_last = input.first().expect("item");

    for num in input.iter() {
        if num > p1_last {
            p1_count += 1;
        }
        p1_last = num;
    }
    println!("Part 1: {}", p1_count);
}

fn part2(input: &Vec<i32>) {
    let mut acc = 0;
    let mut seen = 0;
    let mut last: i32;
    let mut count = 0;

    for (i, num) in input.iter().enumerate() {
        last = acc;
        acc += num;
        if seen == 3 {
            acc -= input[i - seen];
            if last < acc {
                count += 1;
            }
        } else {
            seen += 1;
        }
    }
    println!("Part 2: {}", count);
}
