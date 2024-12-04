advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left_v, mut right_v) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        (left_v.push(nums[0]), right_v.push(nums[1]));
    }

    (left_v.sort(), right_v.sort());

    let sum = left_v
        .iter()
        .zip(right_v.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let (mut left_v, mut right_v) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        (left_v.push(nums[0]), right_v.push(nums[1]));
    }
    for i in left_v {
        let i_32 = i as u32;
        let count = right_v.iter().filter(|&x| x == &i).count();
        sum += i_32 * count as u32;
    }

    Some(sum)
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
