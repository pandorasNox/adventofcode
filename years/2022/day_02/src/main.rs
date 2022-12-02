use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum HandShapes {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("input file path: '{}'", file_path);

    let input_content = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");
    // println!("input content:\n{}", input_content);

    println!("total_score_from_strategy_guide: {}", total_score_from_strategy_guide(input_content.clone()));
    println!("total_score_from_outcome_strategy_guide: {}", total_score_from_outcome_strategy_guide(input_content));
}

fn total_score_from_strategy_guide(input: String) -> u32 {
    let strategy_guide = parse_to_strategy_guide(input).unwrap();
    
    let mut points: u32 = 0;
    for move_strategy in strategy_guide {
        println!("DEBUG total_score_from_strategy_guide game_move:'{:?}'", move_strategy);

        let MoveStrategy(_, should_move) = move_strategy;

        match should_move {
            HandShapes::Rock => {points += 1}
            HandShapes::Paper => {points += 2}
            HandShapes::Scissors => {points += 3}
        }

        match eval_match(move_strategy) {
            Outcome::WIN => {points += 6}
            Outcome::LOSS => {points += 0}
            Outcome::DRAW => {points += 3}
        }
    }

    return points;
}

enum Outcome {
    WIN,
    LOSS,
    DRAW,
}

fn eval_match(ms: MoveStrategy) -> Outcome {
    let MoveStrategy(oppnent_move, should_move) = ms;

    match (oppnent_move, should_move) {
        (HandShapes::Rock, HandShapes::Rock) => Outcome::DRAW,
        (HandShapes::Rock, HandShapes::Paper) => Outcome::WIN,
        (HandShapes::Rock, HandShapes::Scissors) => Outcome::LOSS,
        (HandShapes::Paper, HandShapes::Rock) => Outcome::LOSS,
        (HandShapes::Paper, HandShapes::Paper) => Outcome::DRAW,
        (HandShapes::Paper, HandShapes::Scissors) => Outcome::WIN,
        (HandShapes::Scissors, HandShapes::Rock) => Outcome::WIN,
        (HandShapes::Scissors, HandShapes::Paper) => Outcome::LOSS,
        (HandShapes::Scissors, HandShapes::Scissors) => Outcome::DRAW,
    }
}

#[derive(Debug)]
struct MoveStrategy(HandShapes, HandShapes);

//todo, return Result instead of Option to pass error message
fn parse_to_strategy_guide(input: String) -> Option<Vec<MoveStrategy>> {
    let input_lines = input.split("\n");

    let mut strategy_guide = Vec::<MoveStrategy>::new();
    for (line_nr, line) in input_lines.enumerate() {
        println!("DEBUG parse_to_strategy_guide: line_nr='{}' content='{}'", line_nr, line);

        let line_moves = line.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|m| m.chars().next().unwrap())
            .collect::<Vec<char>>();

        if line_moves.len() != 2 {
            return None;
        }

        let opnents_move = map_to_handshape(*line_moves.first().unwrap()).unwrap();
        let should_play_move = map_to_handshape(*line_moves.last().unwrap()).unwrap();

        strategy_guide.push(MoveStrategy(opnents_move, should_play_move));
    }

    return Some(strategy_guide);
}

// fn allowed_cyphers_opponent() -> [char; 3] {
//     return ['A', 'B', 'C'];
// }

fn map_to_handshape(c: char) -> Option<HandShapes> {
    match c {
        'A' | 'X' => return Some(HandShapes::Rock),
        'B' | 'Y' => return Some(HandShapes::Paper),
        'C' | 'Z' => return Some(HandShapes::Scissors),
        _ => None
    }
}

impl HandShapes {
    fn cipher_opponent(&self) -> char {
        match self {
            HandShapes::Rock => {return 'A';},
            HandShapes::Paper => {return 'B'},
            HandShapes::Scissors => {return 'C'},
        }
    }

    fn cipher_should_play(&self) -> char {
        match self {
            HandShapes::Rock => {return 'X';},
            HandShapes::Paper => {return 'Y'},
            HandShapes::Scissors => {return 'Z'},
        }
    }

    fn score_value(&self) -> u8 {
        match self {
            HandShapes::Rock => {return 1;},
            HandShapes::Paper => {return 2},
            HandShapes::Scissors => {return 3},
        }
    }
}

fn decryp(encryp: String) -> HandShapes {
    return HandShapes::Rock;
}


// ----------------------------------------------------------------------------

#[derive(Debug)]
enum Should {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct OutcomeStrategy(HandShapes, Should);

fn total_score_from_outcome_strategy_guide(input: String) -> u32 {
    let outcome_strategy_guide = parse_to_outcome_strategy_guide(input).unwrap();
    
    let mut points: u32 = 0;
    for move_strategy in outcome_strategy_guide {
        println!("DEBUG total_score_from_strategy_guide game_move:'{:?}'", move_strategy);

        let OutcomeStrategy(oponent, should_outcome) = move_strategy;

        match (oponent, should_outcome) {
            (HandShapes::Rock, Should::Win) => {
                //choose paper
                points += 2;
                //should win
                points += 6;
            }
            (HandShapes::Rock, Should::Lose) => {
                //choose Scissors
                points += 3;
                //should win
                points += 0;
            }
            (HandShapes::Rock, Should::Draw) => {
                //choose Rock
                points += 1;
                //should win
                points += 3;
            }

            (HandShapes::Paper, Should::Win) => {
                //choose Scissors
                points += 3;
                //should win
                points += 6;
            }
            (HandShapes::Paper, Should::Lose) => {
                //choose Rock
                points += 1;
                //should win
                points += 0;
            }
            (HandShapes::Paper, Should::Draw) => {
                //choose paper
                points += 2;
                //should win
                points += 3;
            }
            (HandShapes::Scissors, Should::Win) => {
                //choose rock
                points += 1;
                //should win
                points += 6;
            }
            (HandShapes::Scissors, Should::Lose) => {
                //choose paper
                points += 2;
                //should win
                points += 0;
            }
            (HandShapes::Scissors, Should::Draw) => {
                //choose Scissors
                points += 3;
                //should win
                points += 3;
            }
        }
    }

    return points;
}

// fn score_value(&self) -> u8 {
//     match self {
//         HandShapes::Rock => {return 1;},
//         HandShapes::Paper => {return 2},
//         HandShapes::Scissors => {return 3},
//     }
// }

fn parse_to_outcome_strategy_guide(input: String) -> Option<Vec<OutcomeStrategy>> {
    let input_lines = input.split("\n");

    let mut strategy_guide = Vec::<OutcomeStrategy>::new();
    for (line_nr, line) in input_lines.enumerate() {
        println!("DEBUG parse_to_strategy_guide: line_nr='{}' content='{}'", line_nr, line);

        let line_moves = line.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|m| m.chars().next().unwrap())
            .collect::<Vec<char>>();

        if line_moves.len() != 2 {
            return None;
        }

        let opnents_move = map_to_handshape(*line_moves.first().unwrap()).unwrap();
        let should_play_move = map_to_should_outcome(*line_moves.last().unwrap()).unwrap();

        strategy_guide.push(OutcomeStrategy(opnents_move, should_play_move));
    }

    return Some(strategy_guide);
}

fn map_to_should_outcome(c: char) -> Option<Should> {
    match c {
        'X' => return Some(Should::Lose),
        'Y' => return Some(Should::Draw),
        'Z' => return Some(Should::Win),
        _ => None
    }
}

// ----------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_score_from_strategy_guide() {
        let input = _input_example();

        assert_eq!(15, total_score_from_strategy_guide(input));
    }
}

fn _input_example() -> String {
    return r#"A Y
B X
C Z"#.to_string();

}
