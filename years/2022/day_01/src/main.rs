
fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy)]
struct Elf {
    calories: i32,
}

struct Elfs(Vec<Elf>);

fn heaviest_elf(input: String) -> i32 {
    //let cal_list_per_elf = Regex::new(r"\n\n").unwrap().split(input);
    let input_lines = input.split("\n");

    // let elfs: Vec<Elf> = vec![];
    // let elfs: Vec<Elf> = Vec::new();
    let mut elfs = Vec::<Elf>::new();

    let mut calories_accumulated = Vec::<i32>::new();
    for (line_nr, content) in input_lines.enumerate() {
        println!("line {}:{}\n", line_nr, content);
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

        assert_eq!(24_000, heaviest_elf(input));
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
