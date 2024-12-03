use regex::Regex;

fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug)]
struct MulInstruction {
    x: u64,
    y: u64,
}

fn input_to_instructions(input: &str) -> Vec<MulInstruction> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let instructions: Vec<MulInstruction> = re
        .captures_iter(input)
        .map(|m| {
            let (_, [x, y]) = m.extract();
            MulInstruction {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    instructions
}

pub fn part_1(input: &str) -> u64 {
    let instructions = input_to_instructions(input);
    instructions.into_iter().map(|i: MulInstruction| i.x * i.y).sum()
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 161;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 0;
        assert_eq!(part_2(test_input), expected_result);
    }
}
