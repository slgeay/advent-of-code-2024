advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    });

    left_list.sort();
    right_list.sort();
    Some(
        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    });

    Some(
        left_list
            .iter()
            .map(|left| left * right_list.iter().filter(|right| left == *right).count() as u32)
            .sum::<u32>(),
    )
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
