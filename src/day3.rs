use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum CellType {
    Number,
    Symbol,
    Blank,
}

#[derive(Debug)]
struct Cell {
    class: CellType,
    char: char,
}

impl Cell {
    fn is_gear(&self) -> bool {
        self.char == '*'
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Cell {
            class: match value {
                c if c.is_numeric() => CellType::Number,
                '.' => CellType::Blank,
                _ => CellType::Symbol,
            },
            char: value,
        }
    }
}

struct CellTable(Vec<Vec<Cell>>);

impl CellTable {
    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        match self.0.get(y) {
            Some(bla) => bla.get(x),
            _ => None,
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> CellTable {
    CellTable(
        input
            .split("\n")
            .map(|line| line.chars().map(Cell::from).collect())
            .collect(),
    )
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    line_index: usize,
    index: usize,
    length: usize,
}

impl Number {
    fn from(value: u32, line_index: usize, index: usize, length: usize) -> Number {
        Number {
            value,
            line_index,
            index,
            length,
        }
    }
}

fn get_numbers(cell_table: &CellTable) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];

    cell_table
        .0
        .iter()
        .enumerate()
        .for_each(|(line_index, line)| {
            let mut i = 0;
            while i < line.len() {
                let mut ii = 0;
                let mut number = 0;

                loop {
                    let cell = line.get(i + ii);
                    if cell.is_none() {
                        break;
                    }

                    let cell = cell.unwrap();
                    if cell.class != CellType::Number {
                        break;
                    }

                    number = number * 10;
                    number += cell.char.to_digit(10).unwrap();
                    ii += 1;
                }

                if number != 0 {
                    numbers.push(Number::from(number, line_index, i, ii))
                }

                i += ii + 1;
            }
        });

    numbers
}

fn surrounding_coords_iter(number: &Number) -> impl Iterator<Item = (usize, usize)> + '_ {
    (number.index.saturating_sub(1)..number.index + number.length + 1)
        .map(|x| {
            [
                (number.line_index.saturating_sub(1), x),
                (number.line_index + 1, x),
            ]
        })
        .flatten()
        .chain([
            (number.line_index, number.index.saturating_sub(1)),
            (number.line_index, number.index + number.length),
        ])
}

#[aoc(day3, part1)]
fn part1(input: &CellTable) -> u32 {
    get_numbers(input)
        .iter()
        .filter(|number| {
            surrounding_coords_iter(number).any(|(y, x)| {
                input
                    .get_cell(x, y)
                    .is_some_and(|cell| cell.class == CellType::Symbol)
            })
        })
        .map(|number| number.value)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &CellTable) -> u32 {
    get_numbers(input)
        .iter()
        .filter_map(|number| {
            surrounding_coords_iter(number)
                .find(|(y, x)| input.get_cell(*x, *y).is_some_and(|cell| cell.is_gear()))
                .map(|coord| (coord, number.value))
        })
        .into_group_map_by(|(coord, _)| *coord)
        .iter()
        .map(|(_, group)| group.iter().map(|(_, value)| *value).collect())
        .filter(|values: &Vec<u32>| values.len() == 2)
        .map(|values| values.iter().product::<u32>())
        .sum()
}
