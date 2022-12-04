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
struct Rucksack {
    all_items: Vec<Item>,
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

impl Rucksack {
    fn new(input: String) -> Rucksack {
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

pub fn run() {
    let lines = get_inputs(2022, 3);
    let mut sacks = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        sacks.push(Rucksack::new(line))
    }

    let part1: u32 = sacks
        .iter()
        .map(|s| s.common())
        .map(|item| item.priority)
        .sum();

    let groups: Vec<Vec<Item>> = sacks.iter()
        .map(|x| x.all_items.clone())
        .collect();
    // groups.chunks(3)
    //     .for_each(|chunk| {
    //         let mut common: HashSet<Item> = HashSet::from_iter(chunk.first().unwrap().clone().iter());
    //         for item in chunk {
    //             let item: HashSet<Item> = HashSet::from_iter(item.clone().iter());
    //             common = common.intersection(&item).collect();
    //         }
    //     });
    println!("{:?}", part1)
}
