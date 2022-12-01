use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("total Calories heavies elf is carrying: '{}'", heaviest_elf_calories_carried(contents.clone()));

    let mut elfs_inventory = compose_elf_list_inventory(contents);
    elfs_inventory
      .sort_by(|a, b| a.calories.cmp(&b.calories) );
    elfs_inventory.reverse();

    let first_three_heaviest_elfs = &elfs_inventory[0..=2];

    let mut sum_cals = 0;
    for (_key, elf) in first_three_heaviest_elfs.iter().enumerate() {
        println!("calories: {}", elf.calories);
        sum_cals += elf.calories;
    }

    println!("sum calories: {}", sum_cals);
}

#[derive(Clone, Copy)]
struct Elf {
    calories: i32,
}

fn compose_elf_list_inventory(input: String) -> Vec<Elf> {
    //let cal_list_per_elf = Regex::new(r"\n\n").unwrap().split(input);
    let input_lines = input.split("\n");

    // let elfs: Vec<Elf> = vec![];
    // let elfs: Vec<Elf> = Vec::new();
    let mut elfs = Vec::<Elf>::new();

    let mut calories_accumulated = Vec::<i32>::new();
    for (line_nr, content) in input_lines.enumerate() {
        // println!("line {}:{}\n", line_nr, content);
        if content == "".to_string() {
            //create elf with prev read lines ???
            if calories_accumulated.len() == 0 {
                continue
            }

            let new_elf = Elf {
                calories: calories_accumulated.iter().sum(),
            };

            elfs.push(new_elf);
            calories_accumulated = Vec::<i32>::new();

            continue;
        }

        let calories_number = content.parse::<i32>().unwrap();
        calories_accumulated.push(calories_number);
    }

    //handle EOF case, last elf, as this is not done in the loop
    if calories_accumulated.len() > 0 {
        let new_elf = Elf {
            calories: calories_accumulated.iter().sum(),
        };

        elfs.push(new_elf);
    }

    return elfs;
}

fn heaviest_elf_calories_carried(input: String) -> i32 {
    let elfs = compose_elf_list_inventory(input);

    let index_elf_with_most_cal = elfs
        .iter()
        .enumerate()
        .max_by_key(|(_idx, &elf)| elf.calories)
        .map(|(idx, _elf)| idx).unwrap();

    return elfs[index_elf_with_most_cal].calories;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let input = _input_example();

        assert_eq!(24_000, heaviest_elf_calories_carried(input));
    }
}

fn _input_example() -> String {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#.to_string();

return input;
}
