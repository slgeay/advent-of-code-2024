use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input.trim().split(" ").map(|s| s.parse().unwrap()).collect();
    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for &stone in stones.iter() {
            let stone_str = stone.to_string();
            if stone == 0 {
                new_stones.push(1);
            } else if stone_str.len() % 2 == 0 {
                let (a, b) = stone_str.split_at(stone_str.len() / 2);
                new_stones.extend([a.parse::<u64>().unwrap(), b.parse().unwrap()]);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.trim().split(' ').map(|s| s.parse().unwrap()).collect();
    let rounds: usize = 75;

    let mut mem: HashMap<(usize, u64), u64> = HashMap::new();
    let mut stack: Vec<(usize, u64)> = stones.iter().rev().map(|&s| (rounds, s)).collect::<Vec<_>>();

    while let Some(&(round, stone)) = stack.last() {
        mem.entry((0, stone)).or_insert(1);

        if mem.contains_key(&(round, stone)) {
            stack.pop();
            continue;
        }

        let stone_str = stone.to_string();
        if stone == 0 {
            let new_stone = 1;
            mem.insert((1, stone), 1);
            if let Some(&value) = mem.get(&(round - 1, new_stone)) {
                mem.insert((round, stone), value);
                stack.pop();
            } else {
                stack.push((round - 1, new_stone));
            }
        } else if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();

            mem.insert((1, stone), 2);

            match (mem.get(&(round - 1, left)), mem.get(&(round - 1, right))) {
                (Some(&left_value), Some(&right_value)) => {
                    mem.insert((round, stone), left_value + right_value);
                    stack.pop();
                }
                (left_value, right_value) => {
                    if left_value.is_none() {
                        stack.push((round - 1, left));
                    }
                    if right_value.is_none() {
                        stack.push((round - 1, right));
                    }
                }
            }
        } else {
            let new_stone = stone * 2024;
            mem.insert((1, stone), 1);
            if let Some(&value) = mem.get(&(round - 1, new_stone)) {
                mem.insert((round, stone), value);
                stack.pop();
            } else {
                stack.push((round - 1, new_stone));
            }
        }
    }

    Some(stones.iter().map(|&stone| mem.get(&(rounds, stone)).unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
