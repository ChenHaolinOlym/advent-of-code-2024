advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;

    let mut orders = std::collections::HashMap::new();
    let mut updates = Vec::new();

    let mut ordering = true;

    for line in input.lines() {
        if line == "" {
            ordering = false;
            continue;
        }

        if ordering {
            let sp = line.split("|").collect::<Vec<_>>();
            let first = sp[0].parse::<u32>().unwrap();
            let second = sp[1].parse::<u32>().unwrap();

            orders.entry(first).or_insert(vec![]).push(second);
        } else {
            let sp = line
                .split(",")
                .map(|u| u.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(sp);
        }
    }

    for update in updates {
        let mut correct_order = true;

        for i in 0..update.len() {
            if let Some(orders) = orders.get(&update[i]) {
                for j in 0..i {
                    if orders.contains(&update[j]) {
                        correct_order = false;
                        break;
                    }
                }
            } else {
                continue;
            }
        }

        if correct_order {
            res += update[(update.len() - 1) / 2 as usize];
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Topological sort won't work because the graph contains cycle
    let mut res = 0;

    let mut orders = std::collections::HashMap::new();
    let mut updates = Vec::new();

    let mut ordering = true;

    for line in input.lines() {
        if line == "" {
            ordering = false;
            continue;
        }

        if ordering {
            let sp = line.split("|").collect::<Vec<_>>();
            let first = sp[0].parse::<u32>().unwrap();
            let second = sp[1].parse::<u32>().unwrap();

            orders.entry(first).or_insert(vec![]).push(second);
        } else {
            let sp = line
                .split(",")
                .map(|u| u.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(sp);
        }
    }

    for update in updates {
        let mut correct_order = true;

        for i in 0..update.len() {
            if let Some(orders) = orders.get(&update[i]) {
                for j in 0..i {
                    if orders.contains(&update[j]) {
                        correct_order = false;
                        break;
                    }
                }
            } else {
                continue;
            }
        }

        if !correct_order {
            let mut update = update;
            update.sort_by(|a, b| {
                if orders.get(a).unwrap_or(&vec![]).contains(b) {
                    std::cmp::Ordering::Less
                } else if orders.get(b).unwrap_or(&vec![]).contains(a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            res += update[(update.len() - 1) / 2 as usize];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
