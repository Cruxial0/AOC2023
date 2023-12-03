use std::env;

mod part01;
mod part02;
mod file_helper;

extern crate phf;

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let parameter = &args[2];

    if query == "part" {
        match parameter.as_str() {
            "1" => part01::run(),
            "2" => part02::run(),
            _ => todo!()
        }
    }
    
}