advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut succ_lines = 0;

    let max_change = 3;
    let min_change = 1;

    'outer: for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let start = nums[0];
        let end = nums[nums.len() - 1];

        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        if !(&start == max && &end == min) && !(&start == min && &end == max) {
            continue;
        }
        let mut increasing = true;
        let mut decreasing = true;

        for window in nums.windows(2) {
            let change = window[1] - window[0];

            // Check difference constraints
            if change.abs() > max_change || change.abs() < min_change {
                continue 'outer; // Skip this line if invalid difference found
            }

            // Check for consistent direction
            if change < 0 {
                increasing = false;
            }
            if change > 0 {
                decreasing = false;
            }
        }

        // Check if the line is either fully increasing or fully decreasing
        if increasing || decreasing {
            succ_lines += 1;
        }
    }
    Some(succ_lines)
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("Part two");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
