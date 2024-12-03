fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn input_to_lists(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_increasing_safely(x: u64, y: u64) -> bool {
    (x < y) && (y - x <= 3) && (y - x >= 1)
}

fn is_decreasing_safely(x: u64, y: u64) -> bool {
    (x > y) && (x - y <= 3) && (x - y >= 1)
}

fn is_safe(report: &Vec<u64>) -> bool {
    if report[0] < report[1] {
        report.windows(2).all(|x| is_increasing_safely(x[0], x[1]))
    } else if report[0] > report[1] {
        report.windows(2).all(|x| is_decreasing_safely(x[0], x[1]))
    } else {
        false
    }
}

pub fn part_1(input: &str) -> usize {
    let reports = input_to_lists(input);
    let safe = reports.into_iter().filter(|r| is_safe(&r)).count();
    safe
}

pub fn part_2(input: &str) -> usize {
    let reports = input_to_lists(input);
    let safe = reports
        .into_iter()
        .filter(|r| {
            for i in 0..r.len() {
                let sub_report = [&r[..i], &r[i + 1..]].concat();
                if is_safe(&sub_report) {
                    return true;
                }
            }
            false
        })
        .count();
    safe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 2;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 4;
        assert_eq!(part_2(test_input), expected_result);
    }
}
