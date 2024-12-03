advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let levels = line
                    .split_whitespace()
                    .map(|level| level.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let direction = if levels[1] - levels[0] > 0 { 1 } else { -1 };
                for i in 1..levels.len() {
                    let diff = (levels[i] - levels[i - 1]) * direction;
                    if !(1..4).contains(&diff) {
                        return None;
                    }
                }
                Some(())
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let levels = line
                    .split_whitespace()
                    .map(|level| level.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (0..levels.len())
                    .any(|i| {
                        let mut levels = levels.clone();
                        levels.remove(i);
                        let direction = if levels[1] - levels[0] > 0 { 1 } else { -1 };
                        for i in 1..levels.len() {
                            let diff = (levels[i] - levels[i - 1]) * direction;
                            if !(1..4).contains(&diff) {
                                return false;
                            }
                        }
                        true
                    })
                    .then_some(())
            })
            .count() as u32,
    )
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
