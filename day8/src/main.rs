use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    y: usize,
    x: usize,
}

fn input_to_antenaes(input: &str) -> (usize, usize, HashMap<char, Vec<Coordinate>>) {
    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                let antenna = Coordinate { x, y };

                antennas
                    .entry(c)
                    .and_modify(|l| l.push(antenna))
                    .or_insert(vec![antenna]);
            }
        }
    }

    // calculate edges of the map
    let max_y = input.lines().count();
    let max_x = input.lines().next().map_or(0, |l| l.len());

    (max_x, max_y, antennas)
}

fn antinode(x: i64, y: i64, max_x: usize, max_y: usize) -> Option<Coordinate> {
    if x < 0 || y < 0 || x as usize >= max_x || y as usize >= max_y {
        return None;
    }

    Some(Coordinate {
        y: y as usize,
        x: x as usize,
    })
}

pub fn part_1(input: &str) -> usize {
    let (max_x, max_y, antennas_map) = input_to_antenaes(input);
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for (_, antennas) in antennas_map {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];

                // distance between the two nodes
                let offset_x = a.x as i64 - b.x as i64;
                let offset_y = a.y as i64 - b.y as i64;

                // get possible antinodes
                let antinode_a = antinode(a.x as i64 + offset_x, a.y as i64 + offset_y, max_x, max_y);
                let antinode_b = antinode(b.x as i64 - offset_x, b.y as i64 - offset_y, max_x, max_y);

                if let Some(antinode) = antinode_a {
                    antinodes.insert(antinode);
                }

                if let Some(antinode) = antinode_b {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.into_iter().count()
}

pub fn part_2(input: &str) -> usize {
    let (max_x, max_y, antennas_map) = input_to_antenaes(input);
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for (_, antennas) in antennas_map {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];

                // distance between the two nodes
                let offset_x = a.x as i64 - b.x as i64;
                let offset_y = a.y as i64 - b.y as i64;

                antinodes.insert(a);

                // get possible antinodes
                let mut curr_antinode = a;
                loop {
                    let antinode = antinode(
                        curr_antinode.x as i64 + offset_x,
                        curr_antinode.y as i64 + offset_y,
                        max_x,
                        max_y,
                    );

                    if let Some(antinode) = antinode {
                        antinodes.insert(antinode);
                        curr_antinode = antinode;
                    } else {
                        break;
                    }
                }

                curr_antinode = a;
                loop {
                    let antinode = antinode(
                        curr_antinode.x as i64 - offset_x,
                        curr_antinode.y as i64 - offset_y,
                        max_x,
                        max_y,
                    );

                    if let Some(antinode) = antinode {
                        antinodes.insert(antinode);
                        curr_antinode = antinode;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.into_iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 14;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 34;
        assert_eq!(part_2(&test_input), expected_result);
    }
}
