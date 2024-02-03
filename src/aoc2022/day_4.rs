// Space needs to be cleared before the last supplies can be unloaded from the ships, and so
// several Elves have been assigned the job of cleaning up sections of the camp. Every section has
// a unique ID number, and each Elf is assigned a range of section IDs.
//
// However, as some of the Elves compare their section assignments with each other, they've noticed
// that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated
// effort, the Elves pair up and make a big list of the section assignments for each pair (your
// puzzle input).
//
// ~ Part 1 ~
//
// Some of the pairs have noticed that one of their assignments fully contains the other. For
// example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment
// fully contains the other, one Elf in the pair would be exclusively cleaning sections their
// partner will already be cleaning, so these seem like the most in need of reconsideration.
//
// In how many assignment pairs does one range fully contain the other?
//
// ~ Part 2 ~
//
// It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like
// to know the number of pairs that overlap at all.
//
// In how many assignment pairs do the ranges overlap?

use crate::io;

pub fn solve() -> () {
    let data = io::read_assignments("data/aoc2022/day_4.txt").unwrap();

    let part_1 = data.iter()
        .filter(|pair| check_subset(pair))
        .count();

    let part_2 = data.iter()
        .filter(|pair| check_intersect(pair))
        .count();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn check_subset(pair: &(io::Assignment, io::Assignment)) -> bool {
    (pair.0.lower <= pair.1.lower && pair.0.upper >= pair.1.upper)  // 2 is a subset of 1
        || (pair.0.lower >= pair.1.lower && pair.0.upper <= pair.1.upper)   // 1 is a subset of 2
}

fn check_intersect(pair: &(io::Assignment, io::Assignment)) -> bool {
    (pair.0.upper >= pair.1.lower)      // upper bound overlaps lower bound
        ^ (pair.0.lower > pair.1.upper) // exclude where our lower is also higher than their upper
}

