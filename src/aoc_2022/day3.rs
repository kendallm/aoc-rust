use crate::get_inputs;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Item {
    id: char,
    priority: u32,
}

impl Item {
    fn new(id: char) -> Item {
        let priority = match id.is_uppercase() {
            true => {
                let val = id as u32;
                let start = 'A' as u32;
                val - start + 27
            }
            false => {
                let val = id as u32;
                let start = 'a' as u32;
                val - start + 1
            }
        };
        self::Item { id, priority }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rucksack {
    all_items: Vec<Item>,
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

impl Rucksack {
    pub fn new(input: String) -> Rucksack {
        let items: Vec<Item> = input.chars().map(Item::new).collect();

        let mut compartments = Vec::new();
        items.chunks(items.len() / 2).for_each(|item| {
            compartments.push(Vec::from(item));
        });

        Rucksack {
            all_items: items,
            first_compartment: compartments.first().unwrap().clone(),
            second_compartment: compartments.last().unwrap().clone(),
        }
    }

    fn common(&self) -> Item {
        let left: HashSet<&Item> = HashSet::from_iter(self.second_compartment.iter());
        let right: HashSet<&Item> = HashSet::from_iter(self.first_compartment.iter());
        let common = left.intersection(&right);
        common.last().unwrap().clone().clone()
    }
}
/// So doc tests are dope
/// ```
/// use aoc_rust::aoc_2022::day3::part1;
/// use aoc_rust::aoc_2022::day3::Rucksack;
///
/// let lines = vec![
///     "vJrwpWtwJgWrhcsFMMfFFhFp",
///     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///     "PmmdzqPrVvPwwTWBwg",
///     "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
///     "ttgJtRGJQctTZtZT",
///     "CrZsJsPPZsGzwwsLwLmpwMDw"
/// ];
///
/// let mut sacks = Vec::new();
/// for line in lines {
///     sacks.push(Rucksack::new(line.to_string()))
/// }
/// assert_eq!(157, part1(&sacks))
/// ```
///
pub fn part1(sacks: &Vec<Rucksack>) -> u32 {
    sacks
        .iter()
        .map(|s| s.common())
        .map(|item| item.priority)
        .sum()
}

/// So doc tests are dope
/// ```
/// use aoc_rust::aoc_2022::day3::part2;
/// use aoc_rust::aoc_2022::day3::Rucksack;
///
/// let lines = vec![
///     "vJrwpWtwJgWrhcsFMMfFFhFp",
///     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///     "PmmdzqPrVvPwwTWBwg",
///     "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
///     "ttgJtRGJQctTZtZT",
///     "CrZsJsPPZsGzwwsLwLmpwMDw"
/// ];
///
/// let mut sacks = Vec::new();
/// for line in lines {
///     sacks.push(Rucksack::new(line.to_string()))
/// }
/// assert_eq!(70, part2(&sacks))
/// ```
///
pub fn part2(sacks: &Vec<Rucksack>) -> u32 {
    let mut total = 0;
    sacks.chunks(3).for_each(|chunk| {
        let items: Vec<HashSet<&Item>> = vec![
            HashSet::from_iter(chunk[0].all_items.iter()),
            HashSet::from_iter(chunk[1].all_items.iter()),
            HashSet::from_iter(chunk[2].all_items.iter()),
        ];
        let result = items.iter().fold(items[0].clone(), |acc, xs| {
            acc.intersection(&xs).cloned().collect()
        });

        result.into_iter().for_each(|x| total += x.priority)
    });
    total
}

pub fn run() {
    let lines = get_inputs(2022, 3);
    let mut sacks = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        sacks.push(Rucksack::new(line))
    }

    let part1 = part1(&sacks);
    let part2 = part2(&sacks);
    dbg!(part1);
    dbg!(part2);
}
