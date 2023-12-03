use std::num::ParseIntError;

use phf::phf_map;

use crate::file_helper as fh;

const NUMBERS: phf::Map<&'static str, &'static i32> = phf_map! {
    "one" => &1,
    "two" => &2,
    "three" => &3,
    "four" => &4,
    "five" => &5,
    "six" => &6,
    "seven" => &7,
    "eight" => &8,
    "nine" => &9,
    "zero" => &0
};

pub fn run() {
    let mut total = 0;
    if let Ok(lines) = fh::read_lines(String::from("src\\part01.txt")) {
        for line in lines {
            println!("Parsing line");
            if let Ok(ln) = line {
                // Parse line, and add to total
                if let Ok(parsed_number) = parse_to_number(ln) {
                    total = total + parsed_number;
                }
            }
        }
    }
    println!("result: {}", total);
}

fn parse_to_number(ln: String) -> Result<i32, ParseIntError> {
    let mut numbers: Vec<(usize, String)> = Vec::new();

    // Get index-string pairs
    for key in NUMBERS.keys() {
        if !ln.contains(*key) {
            continue;
        }
        let val: Vec<_> = ln.match_indices(*key).collect::<Vec<_>>();
        let mut new = val
            .iter()
            .map(|(a, b)| (*a, b.to_string()))
            .collect::<Vec<(usize, String)>>();
        numbers.append(&mut new);
    }

    let mut idx = 0;

    for ch in ln.chars() {
        if !ch.is_numeric() {
            idx = idx + 1;
            continue;
        }

        let number: String = ch.to_string();

        numbers.push((idx, number));
        idx = idx + 1;
    }
    // Sort vector in an increasing manner
    numbers.sort_by(|a, b| a.0.cmp(&b.0));

    let num1 = string_to_int(numbers[0].1.to_owned());
    let num2 = string_to_int(numbers[numbers.len() - 1].1.to_owned());
    let combined = num1.to_string() + &num2.to_string();

    println!("computed: {}", combined);

    Ok(combined.parse::<i32>().expect("Failed to parse int"))
}

fn string_to_int(input: String) -> i32 {
    if input.len() == 1 && input.chars().all(|f| f.is_numeric()) {
        return input.parse::<i32>().expect("Failed to parse int");
    }

    
    *NUMBERS[&input]
}
