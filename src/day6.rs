use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::iter::zip;

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let (times, record_distances) = input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<u32>().expect("Invalid Number"))
        })
        .collect_tuple()
        .expect("Invalid Input");

    zip(times, record_distances)
        .map(|(time, record_distance)| {
            (1..time)
                .filter(|x| (time - x) * x > record_distance)
                .count() as u32
        })
        .product::<u32>()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    let (time, record_distance) = input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold(String::new(), |acc, y| acc + y)
                .parse::<u64>()
                .expect("Invalid Number")
        })
        .collect_tuple()
        .expect("Invalid Input");

    (1..time)
        .filter(|x| (time - x) * x > record_distance)
        .count() as u32
}
