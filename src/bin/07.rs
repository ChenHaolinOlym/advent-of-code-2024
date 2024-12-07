advent_of_code::solution!(7);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u128> {
    let mut sum = 0;
    let ops = ["+", "*"];
    let mut equations = HashMap::new();

    for line in input.lines() {
        let mut sp = line.split(":");
        let value = sp.next().unwrap().parse::<u128>().unwrap();
        let formula = sp
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|v| v.parse::<u128>().unwrap())
            .collect::<Vec<_>>();
        equations.insert(value, formula);
    }

    for (value, formula) in equations.into_iter() {
        let combinations = 2_usize.pow((formula.len() - 1) as u32);
        for comb in 0..combinations {
            let mut total = formula[0];

            for i in 0..(formula.len() - 1) {
                let op = ops[(comb >> i) & 1];
                match op {
                    "+" => total += formula[i + 1],
                    "*" => total *= formula[i + 1],
                    _ => panic!("Invalid operator"),
                }
                if total > value {
                    break;
                }
            }

            if total == value {
                sum += value;
                break;
            }
        }
    }

    Some(sum)
}

fn dfs(formula: Vec<usize>, value: usize) -> bool {
    let n = formula.len();
    if n == 1 {
        return formula[0] == value;
    }

    for i in 0..n - 1 {
        let a = formula[i];
        let b = formula[i + 1];

        // +
        let new = a + b;
        if new <= value {
            let mut remaining = Vec::new();
            remaining.push(new);
            remaining.extend_from_slice(&formula[i + 2..]);
            if dfs(remaining.clone(), value) {
                return true;
            }
            remaining.pop();
        }

        // *
        let new = a * b;
        if new <= value {
            let mut remaining = Vec::new();
            remaining.push(new);
            remaining.extend_from_slice(&formula[i + 2..]);
            if dfs(remaining.clone(), value) {
                return true;
            }
            remaining.pop();
        }

        // ||
        let b_length = b.to_string().len();
        let power = 10_usize.pow(b_length as u32);
        let concatenated = a * power + b;
        if concatenated <= value {
            let mut remaining = Vec::new();
            remaining.push(concatenated);
            remaining.extend_from_slice(&formula[i + 2..]);
            if dfs(remaining.clone(), value) {
                return true;
            }
            remaining.pop();
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut equations = HashMap::new();

    for line in input.lines() {
        let mut sp = line.split(":");
        let value = sp.next().unwrap().parse::<usize>().unwrap();
        let formula = sp
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        equations.insert(value, formula);
    }

    for (value, formula) in equations.into_iter() {
        let result = dfs(formula.clone(), value);
        if result {
            sum += value;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
