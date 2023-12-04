use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    chosen_numbers: Vec<u32>,
}

impl Game {
    fn matching_numbers_iter(&self) -> impl Iterator<Item = &u32> {
        self.chosen_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
    }
}

fn get_numbers_from_str(numbers_str: &str) -> Vec<u32> {
    numbers_str
        .split(' ')
        .filter(|x| x != &"")
        .map(|x| x.parse().expect("Invalid number"))
        .collect()
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|line| {
            let (game_str, numbers_str) = line.split_once(": ").expect("Invalid input");
            let (winning_numbers_str, chosen_numbers_str) =
                numbers_str.split_once(" | ").expect("Invalid numbers_str");

            Game {
                id: game_str
                    .rsplit_once(' ')
                    .expect("Invalid game_str")
                    .1
                    .parse()
                    .expect("Invalid game id"),
                winning_numbers: get_numbers_from_str(winning_numbers_str),
                chosen_numbers: get_numbers_from_str(chosen_numbers_str),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| match game.matching_numbers_iter().count() as u32 {
            0 => 0,
            x => 2u32.pow(x - 1),
        })
        .sum()
}

fn explore_cards(games: &Vec<Game>, current_game: &Game, cards_count: &mut u32) {
    current_game
        .matching_numbers_iter()
        .enumerate()
        .for_each(|(i, _)| {
            *cards_count += 1;

            if let Some(new_game) = games.get(current_game.id as usize + i) {
                explore_cards(games, new_game, cards_count)
            }
        });
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut cards_count: u32 = 1;
            explore_cards(input, game, &mut cards_count);

            cards_count
        })
        .sum()
}
