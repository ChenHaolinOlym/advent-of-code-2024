advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;

    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for cap in reg.captures_iter(input) {
        let first = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();

        res += first * second;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;

    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    let mut enabled: bool = true;

    for cap in reg.captures_iter(input) {
        let first = cap.get(1);
        let second = cap.get(2);
        let do_ = cap.get(3);
        let dont = cap.get(4);

        match (first, second, do_, dont) {
            (None, None, Some(_), None) => {
                enabled = true;
            }
            (None, None, None, Some(_)) => {
                enabled = false;
            }
            (Some(first), Some(second), None, None) => {
                if enabled {
                    let first = first.as_str().parse::<u32>().unwrap();
                    let second = second.as_str().parse::<u32>().unwrap();

                    res += first * second;
                }
            }
            _ => unreachable!(),
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
