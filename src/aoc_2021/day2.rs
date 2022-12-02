use std::fs::File;
use std::io::Read;
use std::vec::Vec;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let input = get_inputs();
    let mut depth = 0;
    let mut pos = 0;

    for line in input {
        let mut command = line.split(" ");

        let direction = command.next().expect("unable to get direction");
        if direction == "" {
            continue;
        }
        let units = command.next().expect("unable to get units");
        let value: i32 = units.parse().expect("unable to parse value from unit");
        if direction == "up" {
            depth -= value;
        } else if direction == "down" {
            depth += value;
        } else {
            pos += value
        }
    }
    println!("Part 1: {}", depth * pos);
}

fn part2() {
    let input = get_inputs();
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for line in input {
        let mut command = line.split(" ");

        let direction = command.next().expect("unable to get direction");
        if direction == "" {
            continue;
        }
        let units = command.next().expect("unable to get units");
        let value: i32 = units.parse().expect("unable to parse value from unit");
        if direction == "up" {
            aim -= value;
        } else if direction == "down" {
            aim += value;
        } else {
            pos += value;
            depth = depth + (aim * value);
        }
    }
    println!("Part 1: {}", depth * pos);
}

fn get_inputs() -> Vec<String> {
    let mut input = String::new();
    File::open("aoc_2021/inputs/day2.txt")
        .expect("unable to read file")
        .read_to_string(&mut input)
        .expect("unable to parse file to string");

    input.split("\n").map(|x| String::from(x)).collect()
}
