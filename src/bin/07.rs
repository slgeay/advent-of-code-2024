use std::collections::VecDeque;

advent_of_code::solution!(7);

pub fn part_one(i: &str) -> Option<u64> {
    let lines = i.lines().collect::<Vec<_>>();
    Some(lines.iter().map(|l| {
            let (total, expr) = l.split_once(": ").unwrap();
            let total = total.parse().unwrap();
            let numbers = expr.split(" ").map(|e| e.parse().unwrap()).collect();
            let mut queue: VecDeque<VecDeque<u64>> = VecDeque::new();
            queue.push_back(numbers);
            while queue.len() > 0 {
                let mut numbers = queue.pop_front().unwrap();
                if numbers.len() == 1 {
                    if numbers.pop_front().unwrap() == total {
                        return total;
                    }
                } else {
                    let (a, b) = (numbers.pop_front().unwrap(), numbers.pop_front().unwrap());
                    let mut mul_numbers = numbers.clone();
                    mul_numbers.push_front(a * b);
                    queue.push_front(mul_numbers);
                    numbers.push_front(a + b);
                    queue.push_front(numbers);
                }
            }
            0
        }).sum())
}

pub fn part_two(i: &str) -> Option<u64> {
    let lines = i.lines().collect::<Vec<_>>();
    Some(lines.iter().map(|l| {
            let (total, expr) = l.split_once(": ").unwrap();
            let total = total.parse().unwrap();
            let numbers = expr.split(" ").map(|e| e.parse().unwrap()).collect();
            let mut queue: VecDeque<VecDeque<u64>> = VecDeque::new();
            queue.push_back(numbers);
            while queue.len() > 0 {
                let mut numbers = queue.pop_front().unwrap();
                if numbers.len() == 1 {
                    if numbers.pop_front().unwrap() == total {
                        return total;
                    }
                } else {
                    let (a, b) = (numbers.pop_front().unwrap(), numbers.pop_front().unwrap());
                    let mut mul_numbers = numbers.clone();
                    let mut cat_numbers = numbers.clone();
                    cat_numbers.push_front((a.to_string() + &b.to_string()).parse().unwrap());
                    queue.push_front(cat_numbers);
                    mul_numbers.push_front(a * b);
                    queue.push_front(mul_numbers);
                    numbers.push_front(a + b);
                    queue.push_front(numbers);
                }
            }
            0
        }).sum())
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
