use std::{fs, env, collections::{HashMap, BinaryHeap, HashSet}};
use std::cmp::Ordering;
use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    // println!("DEBUG input: '{:#?}'", &input_contents);
    // println!("DEBUG parse input: '{:#?}'",  parse(&input_contents));
    println!("solution1: '{}'", solution1(parse(&input_contents.clone())));
    println!("solution2: '{}'", solution2(parse(&input_contents)));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn parse(input: &String) -> (Vec<Vec<u8>>, Position, Position) {
    let y_len = input.lines().count();
    let x_len = input.lines().next().unwrap().len();

    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };

    let alphabet_to_height_map: HashMap<char,u8> = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
    ]);

    let mut map = vec![vec![0; x_len]; y_len];
    for (y_pos, line) in input.lines().enumerate() {
        for (x_pos, c) in line.chars().enumerate() {
            let letter = match c {
                'S' => {
                    start.x = x_pos;
                    start.y = y_pos;
                    'a'
                }
                'E' => {
                    end.x = x_pos;
                    end.y = y_pos;
                    'z'
                }
                'a'..='z' => c,
                _ => panic!("Invalid input"),
            };

            let height_val = *alphabet_to_height_map.get(&letter).unwrap();
            map[y_pos][x_pos] = height_val;
        }
    }

    (map, start, end)
}

#[derive(Debug, PartialEq, Eq)]
struct Waypoint {
    steps: u32,
    pos: Position,
}

impl Ord for Waypoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for Waypoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solution1((map, start_pos, end_pos): (Vec<Vec<u8>>, Position, Position)) -> u32 {
    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    prio_queue.push(Reverse(Waypoint{
        steps: 0,
        pos: start_pos,
    }));

    while let Some(Reverse(smallest_steps_wayp)) = prio_queue.pop() {
        // print!("DEBUG - '{:#?}'\n", smallest_steps_wayp);
        let candidates = neighbors(&map, &smallest_steps_wayp.pos);

        if candidates.contains(&end_pos) {
            return smallest_steps_wayp.steps+1;
        }

        for candidate_pos in candidates {
            if visited.insert(candidate_pos) {
                prio_queue.push(Reverse(Waypoint{
                    steps: smallest_steps_wayp.steps+1,
                    pos: candidate_pos,
                }));
            }
        }
    }

    return 0;
}

fn solution2((map, _, end_pos): (Vec<Vec<u8>>, Position, Position)) -> u32 {
    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    for l_pos in all_low_positions(&map).iter() {
        prio_queue.push(Reverse(Waypoint{
            steps: 0,
            pos: *l_pos,
        }));
    }

    while let Some(Reverse(smallest_steps_wayp)) = prio_queue.pop() {
        // print!("DEBUG - '{:#?}'\n", smallest_steps_wayp);
        let candidates = neighbors(&map, &smallest_steps_wayp.pos);

        if candidates.contains(&end_pos) {
            return smallest_steps_wayp.steps+1;
        }

        for candidate_pos in candidates {
            if visited.insert(candidate_pos) {
                prio_queue.push(Reverse(Waypoint{
                    steps: smallest_steps_wayp.steps+1,
                    pos: candidate_pos,
                }));
            }
        }
    }

    return 0;
}

fn all_low_positions(map: &Vec<Vec<u8>>) -> Vec<Position> {
    let mut out = Vec::new();

    for (y_pos, y_col) in map.iter().enumerate() {
        for (x_pos, height) in y_col.iter().enumerate() {
            if *height == 0 {
                out.push(Position {
                    x: x_pos,
                    y: y_pos,
                });
            }
        }
    }

    return out;
}

// neighbors, without yet nowing if we already visited them
fn neighbors(map: &Vec<Vec<u8>>, curr_pos: &Position) -> Vec<Position> {
    let mut candidates: Vec<Position> = Vec::new();

    let curr_height = map[curr_pos.y][curr_pos.x];

    //up
    if curr_pos.y != 0 {
        let height = map[curr_pos.y-1][curr_pos.x];
        if curr_height >= height || curr_height == height-1 {
            candidates.push(Position {
                x: curr_pos.x,
                y: curr_pos.y-1,
            });
        }
    }

    //down
    if curr_pos.y < map.len()-1 {
        let height = map[curr_pos.y+1][curr_pos.x];
        if curr_height >= height || curr_height == height-1 {
            candidates.push(Position {
                x: curr_pos.x,
                y: curr_pos.y+1,
            });
        }
    }

    //left
    if curr_pos.x != 0 {
        let height = map[curr_pos.y][curr_pos.x-1];
        if curr_height >= height || curr_height == height-1 {
            candidates.push(Position {
                x: curr_pos.x-1,
                y: curr_pos.y,
            });
        }
    }

    //right
    if curr_pos.x < map[curr_pos.y].len()-1 {
        let height = map[curr_pos.y][curr_pos.x+1];
        if curr_height >= height || curr_height == height-1 {
            candidates.push(Position {
                x: curr_pos.x+1,
                y: curr_pos.y,
            });
        }
    }

    return candidates;
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse() {
    //     let data = _input_test_data();

    //     assert_eq!(
    //         vec![],
    //         parse(&data)
    //     );
    // }

    #[test]
    fn test_solution1() {
        let data = _input_test_data();

        assert_eq!(31, solution1(parse(&data)));
    }

    #[test]
    fn test_solution2() {
        let data = _input_test_data();

        assert_eq!(29, solution2(parse(&data)));
    }
}

fn _input_test_data() -> String {
return "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".to_string();
}
