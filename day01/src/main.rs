use phf::phf_map;
use std::{fs, path::Path};

static WORD_TO_NUMBER: phf::Map<&str, u32> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn main() {
    let input = fs::read_to_string(Path::new("input.txt")).expect("Unable to read file");

    let calibration_values = recover_calibration_values(&input);
    println!("Calibration values: {calibration_values}");

    let calibration_values = recover_calibration_values_improved(&input);
    println!("Calibration values improved: {calibration_values}");
}

fn recover_calibration_values(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            *first * 10 + *last
        })
        .sum()
}

fn recover_calibration_values_improved(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = find_numbers(line);
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            first * 10 + last
        })
        .sum()
}

fn find_numbers(line: &str) -> Vec<u32> {
    (0..line.len())
        .filter_map(|i| {
            line.chars().nth(i).unwrap().to_digit(10).or_else(|| {
                WORD_TO_NUMBER
                    .entries()
                    .find_map(|(word, &number)| line[i..].starts_with(word).then_some(number))
            })
        })
        .collect()
}
