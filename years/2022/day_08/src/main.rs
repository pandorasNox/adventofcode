use std::{fs, env, collections::HashMap};

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

fn parse(input: &String) -> Vec<Vec<u8>> {
    let out =         input
        .lines()
        .map(|l|
            l.chars()
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .collect()
        )
        .collect()
    ;

    return out;
}

fn solution1(grid: Vec<Vec<u8>>) -> u32 {
    let x_len = grid[0].len();
    let y_len = grid.len();

    let mut count_visible: u32 = 0;
    for y in 0..y_len {
        // if y == 0 || y == x_len-1 {
        //     continue;
        // }

        // println!("DEBUG/solution1 row: {}", y);
        for x in 0..x_len {
            // if x == 0 || x == y_len-1 {
            //     continue;
            // }

            if is_tree_visible((x, y), grid[y][x], &grid) {
                count_visible += 1;
            }

            // println!("DEBUG/solution1 col: {}", x);
        }
    }

    return count_visible;
}

fn is_tree_visible((x, y): (usize, usize), height: u8, grid: &Vec<Vec<u8>>) -> bool {
    let x_len = grid[0].len();
    let y_len = grid.len();

    if x == 0 || x == x_len - 1 {
        return true;
    }
    if y == 0 || y == y_len - 1 {
        return true;
    }

    let mut is_visible_from_west = true;
    for x_prev in (0..x).rev() {
        if height <= grid[y][x_prev] {
            is_visible_from_west = false;
            break;
        }
    }

    let mut is_visible_from_east = true;
    for x_next in x+1..x_len {
        if height <= grid[y][x_next] {
            is_visible_from_east = false;
            break;
        }
    }

    let mut is_visible_from_north = true;
    for y_prev in (0..y).rev() {
        if height <= grid[y_prev][x] {
            is_visible_from_north = false;
            break;
        }
    }

    let mut is_visible_from_south = true;
    for y_next in y+1..y_len {
        if height <= grid[y_next][x] {
            is_visible_from_south = false;
            break;
        }
    }

    return is_visible_from_west || is_visible_from_east || is_visible_from_north || is_visible_from_south;
}

fn solution2(grid: Vec<Vec<u8>>) -> u32 {
    let x_len = grid[0].len();
    let y_len = grid.len();

    let mut tree_scores: Vec<u32> = Vec::new();
    for y in 0..y_len {
        if y == 0 || y == y_len - 1  {
            continue;
        }
        for x in 0..x_len {
            if x == 0 || x == x_len - 1 {
                continue;
            }
            tree_scores.push(tree_scenic_score((x, y), grid[y][x], &grid));
        }
    }

    // println!("DEBUG tree_scenic_scores:\n{:#?}", tree_scores);

    return *tree_scores.iter().max().unwrap();
}

fn tree_scenic_score((x, y): (usize, usize), height: u8, grid: &Vec<Vec<u8>>) -> u32 {
    let x_len = grid[0].len();
    let y_len = grid.len();

    // if x == 0 || x == x_len - 1 {
    //     return 1;
    // }
    // if y == 0 || y == y_len - 1 {
    //     return 1;
    // }

    let mut count_visible_from_west: u32 = 0;
    for x_prev in (0..x).rev() {
        if height > grid[y][x_prev] {
            count_visible_from_west += 1;
        }
        if height <= grid[y][x_prev] {
            count_visible_from_west += 1;
            break
        }
    }

    let mut count_visible_from_east: u32 = 0;
    for x_next in x+1..x_len {
        if height > grid[y][x_next] {
            count_visible_from_east += 1;
            continue;
        }
        if height <= grid[y][x_next] {
            count_visible_from_east += 1;
            break;
        }
    }

    let mut count_visible_from_north: u32 = 0;
    for y_prev in (0..y).rev() {
        if height > grid[y_prev][x] {
            count_visible_from_north += 1;
        }
        if height <= grid[y_prev][x] {
            count_visible_from_north += 1;
            break;
        }
    }

    let mut count_visible_from_south: u32 = 0;
    for y_next in y+1..y_len {
        if height > grid[y_next][x] {
            count_visible_from_south += 1;
        }
        if height <= grid[y_next][x] {
            count_visible_from_south += 1;
            break;
        }
    }

    return count_visible_from_west * count_visible_from_east * count_visible_from_north * count_visible_from_south;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let data = _input_test_data();

        assert_eq!(21, solution1(parse(&data)));
    }

    #[test]
    fn test_solution2() {
        let data = _input_test_data();

        assert_eq!(8, solution2(parse(&data)));
    }
}

fn _input_test_data() -> String {
return "30373
25512
65332
33549
35390".to_string();
}
