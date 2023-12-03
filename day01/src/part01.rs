

use std::num::ParseIntError;

use super::file_helper as fh;
pub fn run() {
    let mut total = 0; // Declare inital variable for end result

    // Check if file exists, then iterate line-by-line
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

    println!("Result: {}", total);
}

fn parse_to_number(ln: String) -> Result<i32, ParseIntError> {
    let mut result = String::from("");
    for ch in ln.chars() {
        if !ch.is_numeric() {
            continue;
        }
        if result.len() == 2 {
            result.remove(1);
        }
        result.push(ch);
    }
    if result.len() == 1 {
        result.push(result.chars().next().unwrap());
    }
    println!("parsed number: {}", result);
    let var_name: Result<i32, ParseIntError> =
        Ok(result.parse::<i32>().expect("Failed to parse int"));
    var_name
}


