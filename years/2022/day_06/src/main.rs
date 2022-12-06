use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    // println!("DEBUG parse input: '{:#?}'",  parse(input_contents));
    println!("solution1: '{}'", solution1(&input_contents));
    println!("solution2: '{}'", solution2(&input_contents));
}

fn solution1(input: &String) -> u32 {
    let (_, i, _) = input.as_bytes().into_iter()
        .fold((VecDeque::<u8>::new(), 0 as u32, false), |(mut q, mut i, mut found), c| {
            if found {
                return (q, i, found);
            }

            if q.len() < 4 {
                q.push_back(*c);
                i += 1;
                return (q, i, found);
            }
            
            q.pop_front();
            q.push_back(*c);

            let q_as_set = q.iter().collect::<HashSet<_>>();

            if q.len() == q_as_set.len() {
                found = true;
            }

            i += 1;
            (q, i, found)
        });
    return i;
}

fn solution2(input: &String) -> u32 {
    let (_, i, _) = input.as_bytes().into_iter()
        .fold((VecDeque::<u8>::new(), 0 as u32, false), |(mut q, mut i, mut found), c| {
            if found {
                return (q, i, found);
            }

            if q.len() < 14 { //only change : D
                q.push_back(*c);
                i += 1;
                return (q, i, found);
            }
            
            q.pop_front();
            q.push_back(*c);

            let q_as_set = q.iter().collect::<HashSet<_>>();

            if q.len() == q_as_set.len() {
                found = true;
            }

            i += 1;
            (q, i, found)
        });
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let data = _input_test_data_solution1();

        for (input, expect_solution1) in data {
            assert_eq!(expect_solution1, solution1(&input));
        }
    }

    #[test]
    fn test_solution2() {
        let data = _input_test_data_solution2();

        for (input, expect_solution1) in data {
            assert_eq!(expect_solution1, solution2(&input));
        }
    }
}

fn _input_test_data_solution1() -> Vec<(String, u32)> {
    let data = vec![
        ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 5 as u32),
        ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 6 as u32),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 10 as u32),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 11 as u32),
    ];
return data;
}

fn _input_test_data_solution2() -> Vec<(String, u32)> {
    let data = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 19 as u32),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 23 as u32),
        ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 23 as u32),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 29 as u32),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 26 as u32),
        ];
return data;
}
