advent_of_code::solution!(4);

fn get(grid: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    *grid.get(row).and_then(|r| r.get(col)).unwrap_or(&' ')
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let matcher = "XMAS";

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions: [(isize, isize); 8] = [
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'X' => {
                    let i = i as isize;
                    let j = j as isize;
                    let c = directions
                        .iter()
                        .filter(|(x, y)| {
                            (1..=3).all(|n| {
                                get(&grid, (i + (x * n)) as usize, (j + (y * n)) as usize)
                                    == matcher.chars().nth(n as usize).unwrap()
                            })
                        })
                        .count();
                    count += c as u32;
                }
                _ => (),
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            match grid[i][j] {
                'A' => {
                    let w1 = [get(&grid, i - 1, j - 1), get(&grid, i + 1, j + 1)];
                    let w2 = [get(&grid, i - 1, j + 1), get(&grid, i + 1, j - 1)];
                    count += [w1, w2]
                        .iter()
                        .all(|w| *w == ['M', 'S'] || *w == ['S', 'M'])
                        as u32;
                }
                _ => (),
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
