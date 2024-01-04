#![feature(iter_next_chunk, iterator_try_collect)]

use std::convert::TryFrom;
use std::{fs, path::Path};

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubesSet>,
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let [id, games] = s
            .split(':')
            .next_chunk()
            .map_err(|_| "Not enough values to unpack.")?;

        let id: u32 = id
            .split_whitespace()
            .last()
            .ok_or("Not enough values to unpack for ID.")?
            .parse()
            .map_err(|_| "Could not decode ID.")?;

        let sets: Vec<CubesSet> = games.split(';').map(CubesSet::try_from).try_collect()?;

        Ok(Game { id, sets })
    }
}

#[derive(Debug)]
struct CubesSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl TryFrom<&str> for CubesSet {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (mut red, mut green, mut blue) = (0, 0, 0);

        for draw in s.split(',') {
            let [qty, color] = draw
                .split_whitespace()
                .next_chunk()
                .map_err(|_| "Not enough values to unpack.")?;

            let qty: u32 = qty.parse().map_err(|_| "Could not parse quantity.")?;

            match color {
                "red" => red = qty,
                "green" => green = qty,
                "blue" => blue = qty,
                _ => Err("Could not decode color.")?,
            };
        }

        Ok(CubesSet { red, green, blue })
    }
}

impl CubesSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        CubesSet { red, green, blue }
    }

    fn is_possible(&self, bag: &CubesSet) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}

fn find_possible_games_sum(bag: &CubesSet, games: &[Game]) -> u32 {
    games
        .iter()
        .flat_map(|game| {
            game.sets
                .iter()
                .all(|set| set.is_possible(bag))
                .then_some(game.id)
        })
        .sum()
}

fn find_power_of_sets(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.sets.iter().fold([0, 0, 0], |mut max, set| {
                max[0] = max[0].max(set.red);
                max[1] = max[1].max(set.green);
                max[2] = max[2].max(set.blue);
                max
            })
        })
        .map(|max| max.iter().product::<u32>())
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("input.txt")).map_err(|_| "Unable to read file.")?;
    let games: Vec<Game> = input.lines().map(Game::try_from).try_collect()?;

    let bag = CubesSet::new(12, 13, 14);
    let total = find_possible_games_sum(&bag, &games);
    println!("Sum of possible sets: {total}.");

    let total = find_power_of_sets(&games);
    println!("Sum of powers of minimal sets: {total}.");

    Ok(())
}
