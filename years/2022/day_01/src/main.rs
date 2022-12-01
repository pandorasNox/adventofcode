
fn main() {
    println!("Hello, world!");
}

fn heaviest_elf(input: String) -> i32 {
    let input_lines = input.split("\n");

    for (line_nr, content) in input_lines.enumerate() {
        println!("line {}:{}\n", line_nr, content);
    }

    return 0;
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
