advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let line = line;
        let sp: Vec<_> = line.split_whitespace().collect();
        left.push(sp[0].parse::<i32>().unwrap());
        right.push(sp[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut count = 0;

    for i in 0..left.len() {
        count += (left[i] - right[i]).abs();
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let line = line;
        let sp: Vec<_> = line.split_whitespace().collect();
        left.push(sp[0].parse::<i32>().unwrap());
        right.push(sp[1].parse::<i32>().unwrap());
    }

    let mut score = 0;
    let mut map = std::collections::HashMap::new();

    for i in 0..right.len() {
        *map.entry(right[i]).or_insert(0) += 1;
    }

    for i in 0..left.len() {
        score += left[i] * map.get(&left[i]).unwrap_or(&0);
    }
    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
