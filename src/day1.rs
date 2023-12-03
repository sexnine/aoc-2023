use aoc_runner_derive::aoc;

fn is_char_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|x| {
            let first_digit_index = x.find(is_char_digit).expect("No digit found");
            let first_digit = x.as_bytes()[first_digit_index] - 48;

            let second_digit_index = x.rfind(is_char_digit).expect("No digit found");
            let second_digit = x.as_bytes()[second_digit_index] - 48;

            (first_digit * 10 + second_digit) as u32
        })
        .sum()
}

// --- Part 2 --- :)

const MATCHES: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn find(x: String, matches: Vec<String>) -> Option<(usize, usize, String)> {
    for i in 0..x.len() {
        'matches: for m in &matches {
            for (m_i, m_char) in m.chars().enumerate() {
                match x.chars().nth(i + m_i) {
                    Some(c) => {
                        if m_char == c {
                            continue;
                        }
                    }
                    _ => {}
                }

                continue 'matches;
            }

            return Some((i, m.len(), m.to_string()));
        }
    }

    return None;
}

fn num_str_to_int(x: String) -> Option<u32> {
    match MATCHES.iter().position(|&r| r == x) {
        Some(i) => Some((i % 10) as u32),
        None => None,
    }
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|x| {
            let (_, _, first_digit_str) = find(
                x.into(),
                MATCHES.to_vec().iter().map(|x| x.to_string()).collect(),
            )
            .expect("No digit found");
            let (_, _, second_digit_str) = find(
                x.chars().rev().collect(),
                MATCHES
                    .to_vec()
                    .iter()
                    .map(|x| x.chars().rev().collect::<String>())
                    .collect(),
            )
            .expect("No digit found");

            let first_digit = num_str_to_int(first_digit_str).expect("what.");
            let second_digit =
                num_str_to_int(second_digit_str.chars().rev().collect()).expect("what.");

            first_digit * 10 + second_digit
        })
        .sum()
}
