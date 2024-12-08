advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Vec::new();

    let mut antennas = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c != '.' {
                        antennas.entry(c).or_insert(Vec::new()).push((row, col));
                    };
                    c
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut antinodes = HashSet::new();

    for (_antenna, positions) in antennas {
        let mut pairs = Vec::new();
        for (i, first) in positions.iter().enumerate() {
            for second in positions.iter().skip(i + 1) {
                pairs.push((first.clone(), second.clone()));
            }
        }
        for ((row1, col1), (row2, col2)) in pairs {
            let (row1, col1, row2, col2) = (row1 as i32, col1 as i32, row2 as i32, col2 as i32);
            let (dr, dc) = (row2 - row1, col2 - col1);
            let (r1, c1) = (row1 - dr, col1 - dc);
            let (r2, c2) = (row2 + dr, col2 + dc);

            if 0 <= r1 && r1 < grid.len() as i32 && 0 <= c1 && c1 < grid[0].len() as i32 {
                antinodes.insert((r1 as usize, c1 as usize));
            }
            if 0 <= r2 && r2 < grid.len() as i32 && 0 <= c2 && c2 < grid[0].len() as i32 {
                antinodes.insert((r2 as usize, c2 as usize));
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Vec::new();

    let mut antennas = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c != '.' {
                        antennas.entry(c).or_insert(Vec::new()).push((row, col));
                    };
                    c
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut antinodes = HashSet::new();

    for (_antenna, positions) in antennas {
        let mut pairs = Vec::new();
        for (i, first) in positions.iter().enumerate() {
            for second in positions.iter().skip(i + 1) {
                pairs.push((first.clone(), second.clone()));
            }
        }
        for ((row1, col1), (row2, col2)) in pairs {
            let (row1, col1, row2, col2) = (row1 as i32, col1 as i32, row2 as i32, col2 as i32);
            let (dr, dc) = (row2 - row1, col2 - col1);
            let (mut r1, mut c1) = (row1 + dr, col1 + dc);
            let (mut r2, mut c2) = (row2 - dr, col2 - dc);

            if 0 <= r1 && r1 < grid.len() as i32 && 0 <= c1 && c1 < grid[0].len() as i32 {
                antinodes.insert((r1 as usize, c1 as usize));
            }
            while 0 <= r1 && r1 < grid.len() as i32 && 0 <= c1 && c1 < grid[0].len() as i32 {
                antinodes.insert((r1 as usize, c1 as usize));
                r1 -= dr;
                c1 -= dc;
            }
            if 0 <= r2 && r2 < grid.len() as i32 && 0 <= c2 && c2 < grid[0].len() as i32 {
                antinodes.insert((r2 as usize, c2 as usize));
            }
            while 0 <= r2 && r2 < grid.len() as i32 && 0 <= c2 && c2 < grid[0].len() as i32 {
                antinodes.insert((r2 as usize, c2 as usize));
                r2 += dr;
                c2 += dc;
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
