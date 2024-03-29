// The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
// the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin
// taking inventory of their supplies. One important consideration is food - in particular, the
// number of Calories each Elf is carrying (your puzzle input).
//
// The Elves take turns writing down the number of Calories contained by the various meals, snacks,
// rations, etc. that they've brought with them, one item per line. Each Elf separates their own
// inventory from the previous Elf's inventory (if any) by a blank line.
//
// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd
// like to know how many Calories are being carried by the Elf carrying the most Calories.
//
// ~ Part 1 ~
//
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
//
// ~ Part 2 ~
//
// By the time you calculate the answer to the Elves' question, they've already realized that the
// Elf carrying the most Calories of food might eventually run out of snacks.
//
// To avoid this unacceptable situation, the Elves would instead like to know the total Calories
// carried by the top three Elves carrying the most Calories. That way, even if one of those Elves
// runs out of snacks, they still have two backups.
//
// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying
// in total?

use std::cmp::Reverse;
use crate::io;

pub fn solve() -> () {
    let data = io::read_matrix("data/aoc2022/day_1.txt").unwrap();

    let mut cals: Vec<u32> = data.iter()
        .map(|x| -> u32 { x.iter().sum() })
        .collect();
    cals.sort_by_key(|w| Reverse(*w));
    
    let part_1: u32 = cals[0];
    let part_2: u32 = cals[0..3].iter().sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

