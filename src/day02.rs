/**
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
 */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn decrypt(char: Option<&str>) -> Option<Hand> {
    // Specify a course of action for each case.
    match char {
        Some("A") | Some("X") => Some(Hand::Rock),
        Some("B") | Some("Y") => Some(Hand::Paper),
        Some("C") | Some("Z") => Some(Hand::Scissors),
        _ => None,
    }
}

fn calc_score(me: &Option<Hand>, them: &Option<Hand>) -> Option<u8> {
    match me {
        Some(Hand::Rock) => match them {
            Some(Hand::Rock) => Some(1 + 3),
            Some(Hand::Paper) => Some(1 + 0),
            Some(Hand::Scissors) => Some(1 + 6),
            _ => None,
        },
        Some(Hand::Paper) => match them {
            Some(Hand::Rock) => Some(2 + 6),
            Some(Hand::Paper) => Some(2 + 3),
            Some(Hand::Scissors) => Some(2 + 0),
            _ => None,
        },
        Some(Hand::Scissors) => match them {
            Some(Hand::Rock) => Some(3 + 0),
            Some(Hand::Paper) => Some(3 + 6),
            Some(Hand::Scissors) => Some(3 + 3),
            _ => None,
        },
        _ => None,
    }
}

pub fn run() {
    println!("Welcome to day2!");
    let mut total_score: u32 = 0;
    if let Ok(lines) = read_lines("./input/day02.txt") {
        for line in lines {
            if let Ok(strategy) = line {
                let mut iter = strategy.split_whitespace();
                let (them, me) = (decrypt(iter.next()), decrypt(iter.next()));
                let score = calc_score(&me, &them);
                println!("{:?} vs {:?} scores {:?}", me, them, score);

                total_score += score.unwrap() as u32;
            }
        }
    }
    println!("Our score {}", total_score);
}
