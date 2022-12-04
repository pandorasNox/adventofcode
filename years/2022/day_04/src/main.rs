use std::env;
use std::fs;

#[macro_use] extern crate scan_fmt;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    // println!("DEBUG parse input: '{:#?}'",  parse(input_contents));
    println!("solution1: '{}'", solution1(parse(&input_contents)));
    println!("solution2: '{}'", solution2(parse(&input_contents)));
}

fn parse(input: &String) -> Vec<(u32,u32,u32,u32)> {
    let parsed = input
        .lines()
        .map(|l| scan_fmt!(l, "{}-{},{}-{}", u32,u32,u32,u32).unwrap())
        .collect();
    return parsed;
}

fn solution1(input: Vec<(u32,u32,u32,u32)>) -> u32 {
    let amount = input
        .iter()
        .filter(
            |(rmin1,rmax1,rmin2,rmax2)|
              ( (rmin2 >= rmin1 && rmax2 <= rmax1) || (rmin1 >= rmin2 && rmax1 <= rmax2) ) 
        )
        .count();
    return amount as u32;
}

fn solution2(input: Vec<(u32,u32,u32,u32)>) -> u32 {
    let amount = input
        .iter()
        .filter(
            |(rmin1,rmax1,rmin2,rmax2)|
              ( rmin1.max(rmin2) <= rmax1.min(rmax2) ) 
        )
        .count();
    return amount as u32;
}
