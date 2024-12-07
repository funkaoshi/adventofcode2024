fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn input_to_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(": ").collect();
            let result = s[0].parse().unwrap();
            let operands = s[1].split_whitespace().map(|d| d.parse().unwrap()).collect();
            Equation { result, operands }
        })
        .collect()
}

fn calculate(final_result: u64, current_result: u64, operands: &[u64], concat: bool) -> bool {
    // all our operations increase the size of the result, so we can abort early
    if current_result > final_result {
        return false;
    }

    // we've consumed all the operands, confirm if we have the correct answer
    if operands.len() == 0 {
        return final_result == current_result;
    }

    // make some closures so the code looks less goofy
    let add = || calculate(final_result, current_result + operands[0], &operands[1..], concat);
    let mul = || calculate(final_result, current_result * operands[0], &operands[1..], concat);
    let cat = || {
        calculate(
            final_result,
            concat_u64(current_result, operands[0]),
            &operands[1..],
            concat,
        )
    };

    // recursively determine if we can solve the equation, using +, * or optional concat
    add() || mul() || concat && cat()
}

// note: function can overflow if a and b are too big
fn concat_u64(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

pub fn part_1(input: &str) -> u64 {
    let equations = input_to_equations(input);
    equations
        .into_iter()
        .filter(|e| {
            calculate(e.result, e.operands[0] + e.operands[1], &e.operands[2..], false)
                || calculate(e.result, e.operands[0] * e.operands[1], &e.operands[2..], false)
        })
        .map(|e| e.result)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let equations = input_to_equations(input);
    equations
        .into_iter()
        .filter(|e| {
            calculate(e.result, e.operands[0] + e.operands[1], &e.operands[2..], true)
                || calculate(e.result, e.operands[0] * e.operands[1], &e.operands[2..], true)
                || calculate(
                    e.result,
                    concat_u64(e.operands[0], e.operands[1]),
                    &e.operands[2..],
                    true,
                )
        })
        .map(|e| e.result)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 3749;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 11387;
        assert_eq!(part_2(test_input), expected_result);
    }
}
