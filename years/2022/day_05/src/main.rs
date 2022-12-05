use std::collections::VecDeque;
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

    println!("DEBUG parse input: '{:#?}'",  parse(input_contents.clone()));
    println!("solution1: '{}'", solution1(parse(input_contents.clone())));
    println!("solution2: '{}'", solution2(parse(input_contents)));
}

fn parse(input: String) -> (Vec<VecDeque<char>>, Vec<(u8, u8, u8)>) {
    let mut split = input.split("\n\n");

    let crates = split.next().unwrap();
    let moves = split.next().unwrap();
    // println!("DEBUG parse input crates: '{:#?}'",  crates);
    // println!("DEBUG parse input moves: '{:#?}'",  moves);

    return (parse_crates(crates), parse_moves(moves));
}

fn parse_crates(crates_in: &str) -> Vec<VecDeque<char>> {
    let lines = crates_in.lines();

    let stacks_amount = lines.clone().last().unwrap()
        .as_bytes().chunks(4).count();
    // println!("DEBUG parse_crates stacks_amount: '{:#?}'",  stacks_amount);


    // let stacks = Vec::<VecDeque<char>>::new();
    let mut stacks = vec![VecDeque::<char>::new(); stacks_amount];
    // println!("DEBUG parse_crates stacks: '{:#?}'",  stacks);
    for line in lines {
        if line.contains(" 1 ") {
            break;
        }

        let crates_cols = line
            .as_bytes()
            .chunks(4)
            .map(|c| c[1]).collect::<Vec<_>>();

        // let out = crates_cols.iter().map(|c| *c as char).collect::<Vec<_>>();
        // println!("DEBUG parse_crates: '{:#?}'",  out);

        for (i, crate_col) in crates_cols.iter().enumerate() {
            if i > stacks_amount {
                continue;
            }
            if *crate_col as char == ' ' {
                continue;
            }

            stacks[i].push_back(*crate_col as char)
        }
    }

    // for c in chunks {
    //     println!("DEBUG parse_crates: '{}'",  c);
    // }

    // return vec![VecDeque::new()];
    return stacks;
}

fn parse_moves(moves: &str) -> Vec<(u8, u8, u8)> {
    return moves
        .lines()
        .map(|l| scan_fmt!(l, "move {d} from {d} to {d}", u8, u8,u8).unwrap())
        .collect();
}

fn solution1(stack_n_moves: (Vec<VecDeque<char>>, Vec<(u8, u8, u8)>)) -> String {
    let (mut stacks, moves) = stack_n_moves;
    for (amount, from, to) in moves {
        for _ in 0..amount {
            let elem = &stacks[from as usize -1].pop_front().unwrap();
            stacks[to as usize-1].push_front(*elem);
        }
    }

    println!("DEBUG solution1: '{:#?}'",  stacks);

    let mut result = "".to_string();
    for stack in stacks {
        result.push(*stack.front().unwrap());
    }

    return result;
}

fn solution2(stack_n_moves: (Vec<VecDeque<char>>, Vec<(u8, u8, u8)>)) -> String {
    let (mut stacks, moves) = stack_n_moves;
    for (amount, from, to) in moves {
        let mut crane_stack = VecDeque::<char>::new();
        for _ in 0..amount {
            let elem = &stacks[from as usize -1].pop_front().unwrap();
            crane_stack.push_front(*elem)
        }

        for a_crate in crane_stack {
            stacks[to as usize-1].push_front(a_crate);
        }
    }

    let mut result = "".to_string();
    for stack in stacks {
        result.push(*stack.front().unwrap());
    }

    return result;
}
