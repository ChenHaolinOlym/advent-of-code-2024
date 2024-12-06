advent_of_code::solution!(6);

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 1;

    let mut grid = vec![];

    let mut position = (0, 0);
    let mut direction = Direction::Up;

    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                position = (i, j);
            }
        }
    }

    grid[position.0][position.1] = 'X';

    loop {
        match direction {
            Direction::Up => {
                if position.0 == 0 {
                    break;
                }
                match grid[position.0 - 1][position.1] {
                    '#' => direction = direction.next(),
                    'X' => position = (position.0 - 1, position.1),
                    _ => {
                        count += 1;
                        position = (position.0 - 1, position.1);
                        grid[position.0][position.1] = 'X';
                    }
                }
            }
            Direction::Down => {
                if position.0 == rows - 1 {
                    break;
                }
                match grid[position.0 + 1][position.1] {
                    '#' => direction = direction.next(),
                    'X' => position = (position.0 + 1, position.1),
                    _ => {
                        count += 1;
                        position = (position.0 + 1, position.1);
                        grid[position.0][position.1] = 'X';
                    }
                }
            }
            Direction::Left => {
                if position.1 == 0 {
                    break;
                }
                match grid[position.0][position.1 - 1] {
                    '#' => direction = direction.next(),
                    'X' => position = (position.0, position.1 - 1),
                    _ => {
                        count += 1;
                        position = (position.0, position.1 - 1);
                        grid[position.0][position.1] = 'X';
                    }
                }
            }
            Direction::Right => {
                if position.1 == cols - 1 {
                    break;
                }
                match grid[position.0][position.1 + 1] {
                    '#' => direction = direction.next(),
                    'X' => position = (position.0, position.1 + 1),
                    _ => {
                        count += 1;
                        position = (position.0, position.1 + 1);
                        grid[position.0][position.1] = 'X';
                    }
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = vec![];

    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let mut passed: HashSet<(usize, usize)> = HashSet::new();

    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                position = (i, j);
            }
        }
    }

    let original_position = position;

    grid[position.0][position.1] = 'X';
    passed.insert(position);

    loop {
        match direction {
            Direction::Up => {
                if position.0 == 0 {
                    break;
                }
                match grid[position.0 - 1][position.1] {
                    '#' => direction = direction.next(),
                    _ => {
                        position = (position.0 - 1, position.1);
                        passed.insert(position);
                    }
                }
            }
            Direction::Down => {
                if position.0 == rows - 1 {
                    break;
                }
                match grid[position.0 + 1][position.1] {
                    '#' => direction = direction.next(),
                    _ => {
                        position = (position.0 + 1, position.1);
                        passed.insert(position);
                    }
                }
            }
            Direction::Left => {
                if position.1 == 0 {
                    break;
                }
                match grid[position.0][position.1 - 1] {
                    '#' => direction = direction.next(),
                    _ => {
                        position = (position.0, position.1 - 1);
                        passed.insert(position);
                    }
                }
            }
            Direction::Right => {
                if position.1 == cols - 1 {
                    break;
                }
                match grid[position.0][position.1 + 1] {
                    '#' => direction = direction.next(),
                    _ => {
                        position = (position.0, position.1 + 1);
                        passed.insert(position);
                    }
                }
            }
        }
    }

    let mut obstacles = 0;

    for &(r, c) in passed.iter() {
        grid[r][c] = '#';

        let mut looped = true;
        direction = Direction::Up;
        position = original_position;

        for _ in 0..passed.len() * 2 {
            match direction {
                Direction::Up => {
                    if position.0 == 0 {
                        looped = false;
                        break;
                    }
                    match grid[position.0 - 1][position.1] {
                        '#' => direction = direction.next(),
                        _ => position = (position.0 - 1, position.1),
                    }
                }
                Direction::Down => {
                    if position.0 == rows - 1 {
                        looped = false;
                        break;
                    }
                    match grid[position.0 + 1][position.1] {
                        '#' => direction = direction.next(),
                        _ => position = (position.0 + 1, position.1),
                    }
                }
                Direction::Left => {
                    if position.1 == 0 {
                        looped = false;
                        break;
                    }
                    match grid[position.0][position.1 - 1] {
                        '#' => direction = direction.next(),
                        _ => position = (position.0, position.1 - 1),
                    }
                }
                Direction::Right => {
                    if position.1 == cols - 1 {
                        looped = false;
                        break;
                    }
                    match grid[position.0][position.1 + 1] {
                        '#' => direction = direction.next(),
                        _ => position = (position.0, position.1 + 1),
                    }
                }
            }
        }

        if looped {
            obstacles += 1;
        }

        grid[r][c] = 'X';
    }

    Some(obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
