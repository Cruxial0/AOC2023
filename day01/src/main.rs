use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
fn main() {
    
    let mut total = 0; // Declare inital variable for end result

    // Check if file exists, then iterate line-by-line
    if let Ok(lines) = read_lines("./day1.txt"){
        for line in lines{
            if let Ok(ln) = line{
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
    for ch in ln.chars(){
        if result.len() == 2 { result.remove(1); }
        result.push(ch);
    }
    let var_name: Result<i32, ParseIntError> = Ok(result.parse::<i32>().expect("Failed to parse int"));
    var_name
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}