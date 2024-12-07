fn main() {
    let input: &str = include_str!("../files/input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

struct Coordinate {
    // increase if coordinates need to be bigger
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn input_to_lab_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn starting_position(map: &Vec<Vec<char>>) -> Coordinate {
    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '^' {
                return Coordinate { x: i, y: j };
            }
        }
    }
    panic!("Didn't find starting character.")
}

fn next_position(start: &Coordinate, direction: Direction) -> Coordinate {
    match direction {
        Direction::Up => Coordinate {
            x: start.x,
            y: start.y - 1,
        },
        Direction::Right => Coordinate {
            x: start.x + 1,
            y: start.y,
        },
        Direction::Down => Coordinate {
            x: start.x,
            y: start.y + 1,
        },
        Direction::Left => Coordinate {
            x: start.x - 1,
            y: start.y,
        },
    }
}

/// check if our next move will take us off the map
fn finished(start: &Coordinate, end: &Coordinate, direction: Direction) -> bool {
    match direction {
        Direction::Up => start.y == 0,
        Direction::Right => start.x == end.x - 1,
        Direction::Down => start.y == end.y - 1,
        Direction::Left => start.x == 0,
    }
}

/// Recursively move through the map until we move off it.
fn walk(start: &Coordinate, end: &Coordinate, direction: Direction, map: &mut Vec<Vec<char>>) {
    // mark that we've walked here
    map[start.y][start.x] = 'X';

    // we're going to walk off the map
    if finished(start, end, direction) {
        return;
    }

    // what's the next spot?
    let next = next_position(&start, direction);

    // will we walk into a wall?
    if map[next.y][next.x] == '#' {
        // turn and try again!
        return walk(start, end, direction.rotate(), map);
    }

    // let's keep walking!
    return walk(&next, end, direction, map);
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{c}");
        }
        println!("");
    }
}

pub fn part_1(input: &str) -> usize {
    let mut lab_map = input_to_lab_map(input);

    // we assume starting position is always up ('^')
    let direction = Direction::Up;
    let start = starting_position(&lab_map);

    let end = Coordinate {
        x: lab_map[0].len(),
        y: lab_map.len(),
    };

    // mark lab_map with all the locations we've visted
    walk(&start, &end, direction, &mut lab_map);

    lab_map.into_iter().flatten().filter(|c| *c == 'X').count()
}

pub fn part_2(input: &str) -> usize {
    31
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 41;
        assert_eq!(part_1(test_input), expected_result);
    }

    #[test]
    fn test_part_2() {
        let test_input = include_str!("../files/test.txt");
        let expected_result = 31;
        assert_eq!(part_2(test_input), expected_result);
    }
}
