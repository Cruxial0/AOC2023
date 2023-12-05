use std::num::ParseIntError;

use crate::{
    data::{self, Game, Set},
    file_helper::read_lines,
};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn run() {
    let mut total = 0;
    if let Ok(lines) = read_lines(String::from("src\\source.txt")) {
        for line in lines {
            if let Ok(ln) = line {
                if let Ok(res) = parse_game(ln) {
                    total += res;
                }
            }
        }
    }
    println!("total: {}", total)
}

fn parse_game(ln: String) -> Result<i32, ParseIntError> {
    let mut game = Game::new();
    let input = ln.split(':').last();
    let id = ln
        .split(":")
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<i32>()
        .expect("failed to parse int");
    game.id = id;
    let mut current_number = 0;

    for set in input.unwrap().split(';') {
        let mut dataset = Set::new();
        for color in set.replace(",", "").split(' ') {
            if color.len() == 0 {
                continue;
            }
            match color {
                "red" => {
                    if current_number > dataset.num_red {
                        dataset.num_red = current_number;
                    }
                }
                "green" => {
                    if current_number > dataset.num_green {
                        dataset.num_green = current_number;
                    }
                }
                "blue" => {
                    if current_number > dataset.num_blue {
                        dataset.num_blue = current_number;
                    }
                }
                y => current_number = y.parse::<i32>().expect("failed to parse int"),
            }
        }
        game.sets.push(dataset);
    }

    if game.is_valid(MAX_RED, MAX_GREEN, MAX_BLUE) {
        return Ok(game.id);
    }
    Ok(0)
}
