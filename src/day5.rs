use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct MapRange {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl MapRange {
    fn source_end(&self) -> u32 {
        self.source_start + self.length - 1
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn destination_of(&self, source: u32) -> u32 {
        self.ranges
            .iter()
            .find_map(
                |range| match range.source_start <= source && source <= range.source_end() {
                    true => Some(range.destination_start + (source - range.source_start)),
                    false => None,
                },
            )
            .unwrap_or(source)
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Input {
    fn part_two_seeds_iter(&self) -> Box<dyn Iterator<Item = u32>> {
        let mut ranges_iter = self
            .seeds
            .chunks(2)
            .map(|numbers| numbers[0]..numbers[0] + numbers[1]);

        let mut iter: Box<dyn Iterator<Item = u32>> = Box::new(ranges_iter.next().unwrap());

        for range in ranges_iter {
            iter = Box::new(iter.chain(range));
        }

        iter
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let (seeds_str, maps_str) = input.split_once("\n\n").expect("Invalid Input");

    Input {
        seeds: seeds_str[7..]
            .split(' ')
            .map(|x| x.parse().expect("Invalid Number"))
            .collect(),
        maps: maps_str
            .split("\n\n")
            .map(|map_str| Map {
                ranges: map_str
                    .split('\n')
                    .skip(1)
                    .map(|numbers_str| {
                        let mut iter = numbers_str
                            .split(' ')
                            .map(|y| y.parse().expect("Invalid Number"));

                        MapRange {
                            destination_start: iter.next().expect("Invalid Range"),
                            source_start: iter.next().expect("Invalid Range"),
                            length: iter.next().expect("Invalid Range"),
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn get_lowest_location(seeds_iter: impl Iterator<Item = u32>, maps: &Vec<Map>) -> u32 {
    seeds_iter
        .map(|seed| maps.iter().fold(seed, |num, map| map.destination_of(num)))
        .min()
        .expect("uhm..")
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    get_lowest_location(input.seeds.iter().map(|x| *x), &input.maps)
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    get_lowest_location(input.part_two_seeds_iter(), &input.maps)
}
