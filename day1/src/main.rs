use std::iter::zip;

fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn split_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .map(|v: Vec<&str>| v.into_iter().map(|x| x.parse().unwrap()).collect())
        .map(|v: Vec<i64>| (v[0], v[1]))
        .unzip()
}

pub fn part_1(input: &str) -> i64 {
    // split pairs of numbers into two lists
    let (mut left, mut right) = split_lists(input);

    left.sort();
    right.sort();

    // the distance between the two lists is the sum of the difference of each
    // pair of numbers
    let difference = zip(left, right).map(|(x, y)| (x - y).abs()).sum();

    difference
}

pub fn part_2(input: &str) -> i64 {
    let (left, right) = split_lists(input);

    // the similarity between the two lists is the sum of the number on left
    // multiplied by the number of times it appears in the list on on the right.
    let similarity = left
        .into_iter()
        // todo: how do you avoid this clone!
        .map(|x| x * right.clone().into_iter().filter(|y| x == *y).count() as i64)
        .sum();

    similarity
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 11;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 31;
        assert_eq!(part_2(test_input), expected_result);
    }
}
