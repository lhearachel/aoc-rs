// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the
// snack storage, a giant Rock Paper Scissors tournament is already in progress.
//
// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round,
// the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then,
// a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper
// defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//
// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
// input) that they say will be sure to help you win. "The first column is what your opponent is
// going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the
// Elf is called away to help with someone's tent.
//
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper,
// and Z for Scissors. Winning every time would be suspicious, so the responses must have been
// carefully chosen.
//
// The winner of the whole tournament is the player with the highest score. Your total score is the
// sum of your scores for each round. The score for a single round is the score for the shape you
// selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the
// round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//
// ~ Part 1 ~
//
// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
// score you would get if you were to follow the strategy guide.
//
// What would your total score be if everything goes exactly according to your strategy guide?
//
// ~ Part 2 ~
//
// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column
// says how the round needs to end: X means you need to lose, Y means you need to end the round in
// a draw, and Z means you need to win. Good luck!"
//
// The total score is still calculated in the same way, but now you need to figure out what shape
// to choose so the round ends as indicated.
//
// Following the Elf's instructions for the second column, what would your total score be if
// everything goes exactly according to your strategy guide?

use std::iter::Sum;
use lazy_static::lazy_static;
use crate::io;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
    Invalid = u8::MAX,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum Points {
    Win = 6,
    Lose = 0,
    Draw = 3,
    Invalid = u8::MAX,
}

struct RoundScore {
    imp: u32,
    exp: u32,
}

impl Sum for RoundScore {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>
    {
        iter.fold(RoundScore { imp: 0, exp: 0 },
            |a, b| Self { imp: a.imp + b.imp, exp: a.exp + b.exp }
        )
    }
}

lazy_static! {
    static ref SCORES: Vec<Vec<Points>> = vec![
        vec![Points::Draw, Points::Lose, Points::Win ],
        vec![Points::Win,  Points::Draw, Points::Lose],
        vec![Points::Lose, Points::Win,  Points::Draw],
    ];
}

pub fn solve() -> () {
    let data = io::read_str_pairs("data/aoc2022/day_2.txt").unwrap();

    let all_rounds: RoundScore = data.iter()
        .map(|round| score(round))
        .sum();

    println!("Part 1: {0}", all_rounds.imp);
    println!("Part 2: {0}", all_rounds.exp);
}

fn score(round: &(String, String)) -> RoundScore {
    let them = match round.0.as_str() {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => RPS::Invalid,
    };

    let us = match round.1.as_str() {
        "X" => (RPS::Rock,     Points::Lose),
        "Y" => (RPS::Paper,    Points::Draw),
        "Z" => (RPS::Scissors, Points::Win),
        _ =>   (RPS::Invalid,  Points::Invalid),
    };

    if them == RPS::Invalid || us.0 == RPS::Invalid || us.1 == Points::Invalid {
        return RoundScore { imp: u32::MAX, exp: u32::MAX };
    }

    let pick = match (them, us.1) {
        (RPS::Rock,     Points::Lose) => RPS::Scissors,
        (RPS::Rock,     Points::Draw) => RPS::Rock,
        (RPS::Rock,     Points::Win ) => RPS::Paper,
        (RPS::Paper,    Points::Lose) => RPS::Rock,
        (RPS::Paper,    Points::Draw) => RPS::Paper,
        (RPS::Paper,    Points::Win ) => RPS::Scissors,
        (RPS::Scissors, Points::Lose) => RPS::Paper,
        (RPS::Scissors, Points::Draw) => RPS::Scissors,
        (RPS::Scissors, Points::Win ) => RPS::Rock,
        (_, _) => RPS::Invalid,
    };

    RoundScore {
        imp: us.0 as u32 + 1 + SCORES[us.0 as usize][them as usize] as u32,
        exp: us.1 as u32 + pick as u32 + 1,
    }
}

