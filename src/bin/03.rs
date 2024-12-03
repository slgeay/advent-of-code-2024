use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(i: &str) -> Option<u32> {
    Some(Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures_iter(i).map(|c|{c.iter().map(|x|x.unwrap().as_str().parse::<u32>().unwrap_or(1)).product::<u32>()}).sum())
}

pub fn part_two(i: &str) -> Option<u32> {
    Some(Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures_iter(&Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap().replace_all(i,"")).map(|c|{c.iter().map(|x|x.unwrap().as_str().parse::<u32>().unwrap_or(1)).product::<u32>()}).sum())
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
