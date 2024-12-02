advent_of_code::solution!(2);

fn check_safe(report: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut increase = true;

    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];

        if i == 0 {
            increase = a < b;
        } else {
            if increase != (a < b) {
                safe = false;
                break;
            }
        }

        if (a - b).abs() < 1 || (a - b).abs() > 3 {
            safe = false;
            break;
        }
    }

    safe
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();

        reports.push(report);
    }

    let mut count = 0;

    for report in reports {
        let safe = check_safe(&report);

        if safe {
            count += 1;
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();

        reports.push(report);
    }

    let mut count = 0;

    for report in reports {
        match (0..report.len()).any(|i| {
            let mut relaxed_report = report.clone();
            relaxed_report.remove(i);

            check_safe(&relaxed_report)
        }) {
            true => {
                count += 1;
            }
            _ => (),
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
