use std::collections::VecDeque;

advent_of_code::solution!(7);

pub fn part_one(i: &str) -> Option<u64> {
    let lines = i.lines().collect::<Vec<_>>();
    let fns = vec![
        |a: u64, b: u64| a + b,
        |a: u64, b: u64| a * b,
    ];
    Some(
        lines
            .iter()
            .map(|l| {
                let (total, expr) = l.split_once(": ").unwrap();
                let total = total.parse().unwrap();
                let numbers = expr.split(" ").map(|e| e.parse().unwrap()).collect::<Vec<u64>>();
                let length = numbers.len();
                let mut queue: VecDeque<(usize, u64)> = VecDeque::new();
                queue.push_back((1, numbers[0]));
                while !queue.is_empty() {
                    let current = queue.pop_front().unwrap();
                    if current.0 == length {
                        if current.1 == total {
                            return total;
                        }
                    } else if current.1 < total {
                        let (a, b) = (current.1, numbers[current.0]);
                        fns.iter().for_each(|fn_| {
                            queue.push_back((current.0 + 1, fn_(a, b)));
                        })
                    }
                }
                0
            })
            .sum(),
    )
}
pub fn part_two(i: &str) -> Option<u64> {
    let lines = i.lines().collect::<Vec<_>>();
    let fns = vec![
        |a: u64, b: u64| a + b,
        |a: u64, b: u64| a * b,
        |a: u64, b: u64| (a.to_string() + &b.to_string()).parse().unwrap(),
    ];
    Some(
        lines
            .iter()
            .map(|l| {
                let (total, expr) = l.split_once(": ").unwrap();
                let total = total.parse().unwrap();
                let numbers = expr.split(" ").map(|e| e.parse().unwrap()).collect::<Vec<u64>>();
                let length = numbers.len();
                let mut queue: VecDeque<(usize, u64)> = VecDeque::new();
                queue.push_back((1, numbers[0]));
                while !queue.is_empty() {
                    let current = queue.pop_front().unwrap();
                    if current.0 == length {
                        if current.1 == total {
                            return total;
                        }
                    } else if current.1 < total {
                        let (a, b) = (current.1, numbers[current.0]);
                        fns.iter().for_each(|fn_| {
                            queue.push_back((current.0 + 1, fn_(a, b)));
                        })
                    }
                }
                0
            })
            .sum(),
    )
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
