// One Elf has the important job of loading all of the rucksacks with supplies for the jungle
// journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few
// items now need to be rearranged.
//
// Each rucksack has two large compartments. All items of a given type are meant to go into exactly
// one of the two compartments. The Elf that did the packing failed to follow this rule for exactly
// one item type per rucksack.
//
// The Elves have made a list of all of the items currently in each rucksack (your puzzle input),
// but they need your help finding the errors. Every item type is identified by a single lowercase
// or uppercase letter (that is, a and A refer to different types of items).
//
// The list of items for each rucksack is given as characters all on a single line. A given
// rucksack always has the same number of items in each of its two compartments, so the first half
// of the characters represent items in the first compartment, while the second half of the
// characters represent items in the second compartment.
//
// To help prioritize item rearrangement, every item type can be converted to a priority:
//  - Lowercase item types a through z have priorities 1 through 26.
//  - Uppercase item types A through Z have priorities 27 through 52.
//
// ~ Part 1 ~
//
// Find the item type that appears in both compartments of each rucksack. What is the sum of the
// priorities of those item types?
//
// ~ Part 2 ~
//
// As you finish identifying the misplaced items, the Elves come to you with another issue.
//
// For safety, the Elves are divided into groups of three. Every Elf carries a badge that
// identifies their group. For efficiency, within each group of three Elves, the badge is the only
// item type carried by all three Elves. That is, if a group's badge is item type B, then all three
// Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be
// carrying any other item type.
//
// The problem is that someone forgot to put this year's updated authenticity sticker on the badges.
// All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be
// attached.
//
// Additionally, nobody wrote down which item type corresponds to each group's badges. The only way
// to tell which item type is the right one is by finding the one item type that is common between
// all three Elves in each group.
//
// Every set of three lines in your list corresponds to a single group, but each group can have a
// different badge item type.
//
// Find the item type that corresponds to the badges of each three-Elf group. What is the sum of
// the priorities of those item types?

use std::collections::BTreeSet;
use crate::io;

pub fn solve() -> () {
    let data = io::read_lines("data/aoc2022/day_3.txt").unwrap();

    let part_1: u32 = data.iter()
        .map(|s| common_item(s.to_string()))
        .map(|c| item_prio(c))
        .sum();

    let part_2: u32 = data.chunks(3)
        .map(|chunk| badge_type(chunk))
        .map(|c| item_prio(c))
        .sum();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn common_item(items: String) -> char {
    let pivot = items.chars().count() / 2;
    let (p1, p2) = items.split_at(pivot);

    let h1: BTreeSet<char> = p1.to_string().chars().collect();
    let h2: BTreeSet<char> = p2.to_string().chars().collect();

    return *(h1.intersection(&h2).next().unwrap());
}

fn badge_type(group: &[String]) -> char {
    let sets: Vec<BTreeSet<char>> = group.iter()
        .map(|s| s.chars().collect())
        .collect();

    sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
        acc.retain(|item| set.contains(item));
        acc
    }).iter().next().cloned().unwrap()
}

fn item_prio(c: char) -> u32 {
    let p = (c as u8) - ('A' as u8);

    if p > 25 { // lowercase
        ((c as u8) - ('a' as u8) + 1) as u32
    } else {    // uppercase
        (p as u32) + 27
    }
}

