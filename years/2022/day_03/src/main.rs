use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    println!("sum_of_priorities_of_same_item_in_both_compartments: '{}'", _sum_of_priorities_of_same_item_in_both_compartments(input_contents.clone()));
    println!("sum_of_badges: '{}'", sum_of_badges(input_contents.clone()));
}

fn _sum_of_priorities_of_same_item_in_both_compartments(_input: String) -> u32 {

    let in_line = _input.split("\n");

    let mut dup_list: Vec<char> = Vec::<char>::new();
    let mut _dup_map: HashMap<char, bool> = HashMap::new();
    'outer: for line in in_line {
        let line_chars = line.chars().collect::<Vec<char>>();

        let mut comp_chunks = line_chars.chunks(line_chars.len()/2);

        let comp1 = comp_chunks.next().unwrap();
        let comp2 = comp_chunks.next().unwrap();
        // println!("DEBUG slice: '{:?}'", comp1);
        // println!("DEBUG slice: '{:?}'", comp2);

        for letter in comp1 {
            if let Some(found_dup_char) = comp2.iter().find(|c| letter == *c) {
                dup_list.push(*found_dup_char);
                _dup_map.insert(*found_dup_char, true);
                continue 'outer;
            }
        }
    }

    // println!("DEBUG dup_list: '{:?}'", dup_map);


    // let duplicated_elem = cmap.iter()
    // .find_map(|(key, &val)| if val > 1 { Some(key) } else { None });

    let mut priority_sum = 0;
    for letter in dup_list {
        // if let Some(letter) = duplicated_elem {}
        // println!("DEBUG {:?}!", 'a' as u32 - 96);
        if letter as u32 > 90 {
            // println!("DEBUG {:?}!", *letter as u32 - 96);
            priority_sum += letter as u32 - 96;
        }
        if letter as u32 <= 90 {
            // println!("DEBUG {:?}!", *letter as u32 - 65 + 27);
            priority_sum += letter as u32 - 65 + 27;
        }
    }

    return priority_sum;
}

fn sum_of_badges(input: String) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let line_chunks_iter = lines.chunks(3);

    let mut dup_list: Vec<char> = Vec::<char>::new();
    'outer: for chunk in line_chunks_iter {
        // println!("DEBUG {:?}!", chunk);

        // let line1: Vec<char> = chunk.first().unwrap().chars().collect();
        let line2: Vec<char> = chunk.get(1).unwrap().chars().collect();
        let line3: Vec<char> = chunk.get(2).unwrap().chars().collect();

        for letter in chunk.first().unwrap().chars().collect::<Vec<char>>() {
            let maybe_in_2 = line2.iter().find(|l| **l == letter);
            let maybe_in_3 = line3.iter().find(|l| **l == letter);

            match (maybe_in_2, maybe_in_3) {
                (Some(_), Some(_)) => {
                    dup_list.push(letter);
                    continue 'outer;
                }
                _ => {continue;}
            }
        }
    }

    let mut priority_sum = 0;
    for letter in dup_list {
        // if let Some(letter) = duplicated_elem {}
        // println!("DEBUG {:?}!", 'a' as u32 - 96);
        if letter as u32 > 90 {
            // println!("DEBUG {:?}!", *letter as u32 - 96);
            priority_sum += letter as u32 - 96;
        }
        if letter as u32 <= 90 {
            // println!("DEBUG {:?}!", *letter as u32 - 65 + 27);
            priority_sum += letter as u32 - 65 + 27;
        }
    }

    return priority_sum;
}
