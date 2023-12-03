use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
struct Handfull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Handfull {
    fn add_from_str(&mut self, color: &str, count: u32) {
        let field = match color {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => {
                panic!("invalid color :(")
            }
        };

        *field += count;
    }
}

impl Default for Handfull {
    fn default() -> Self {
        Handfull {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    handfulls: Vec<Handfull>,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|game_line| {
            let (game_str, outcome_str) = game_line.split_once(": ").expect("whgat.");

            Game {
                id: game_str[5..].parse::<u32>().expect("wha3."),
                handfulls: outcome_str
                    .split("; ")
                    .map(|handfull_str| {
                        let mut handfull = Handfull::default();

                        handfull_str.split(", ").for_each(|group_str| {
                            let (count_str, color) = group_str.split_once(" ").expect("heh?");
                            let count: u32 = count_str.parse().expect("wha t");

                            handfull.add_from_str(color, count);
                        });

                        handfull
                    })
                    .collect(),
            }
        })
        .collect()
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[aoc(day2, part1)]
fn part1(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .filter_map(|game| {
            if game
                .handfulls
                .iter()
                .all(|y| y.red <= MAX_RED && y.green <= MAX_GREEN && y.blue <= MAX_BLUE)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut max_handfull = game.handfulls.get(0).expect(":3").clone();

            game.handfulls.iter().skip(1).for_each(|handfull| {
                for (a, b) in [
                    (&mut max_handfull.red, handfull.red),
                    (&mut max_handfull.green, handfull.green),
                    (&mut max_handfull.blue, handfull.blue),
                ] {
                    if b > *a {
                        *a = b;
                    }
                }
            });

            max_handfull.red * max_handfull.green * max_handfull.blue
        })
        .sum()
}
