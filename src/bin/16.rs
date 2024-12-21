use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let w = input.find('\n').unwrap() as i32 + 1;
    let s = input.find('S').unwrap() as i32;
    let e = input.find('E').unwrap() as i32;
    let limits = 0..input.len() as i32;
    let grid = input.chars().collect::<Vec<_>>();

    let mut priority_queue = BinaryHeap::<(i64, i32, i32)>::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    priority_queue.push((0, s, 1));

    loop {
        let (priority, pos, dir) = priority_queue.pop()?;
        let cost = -priority;
        if pos == e {
            return Some(cost as u64);
        }
        if !visited.insert((pos, dir)) {
            continue;
        }

        let next = pos + dir;
        if limits.contains(&next) && grid[next as usize] != '#' {
            priority_queue.push((priority - 1, next, dir));
        }

        for dir in {
            if dir == 1 || dir == -1 {
                [w, -w]
            } else {
                [1, -1]
            }
        } {
            priority_queue.push((priority - 1000, pos, dir));
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let w = input.find('\n').unwrap() as i32 + 1;
    let s = input.find('S').unwrap() as i32;
    let e = input.find('E').unwrap() as i32;
    let limits = 0..input.len() as i32;
    let grid = input.chars().collect::<Vec<_>>();

    let mut priority_queue = BinaryHeap::<(i64, i32, i32, Vec<i32>)>::new();
    let mut visited = HashMap::<(i32, i32), i64>::new();
    priority_queue.push((0, s, 1, Vec::<i32>::new()));

    let mut best_cost = i64::MAX;
    let mut best_tiles = HashSet::<i32>::new();

    loop {
        let (priority, pos, dir, path) = priority_queue.pop()?;
        let cost = -priority;
        if cost > best_cost {
            return Some(best_tiles.len() as u64 + 1);
        }
        if pos == e {
            best_cost = cost;
            best_tiles.extend(path.iter().cloned());
            continue;
        }

        if let Some(&old_cost) = visited.get(&(pos, dir)) {
            if old_cost < cost {
                continue;
            }
        } else {
            visited.insert((pos, dir), cost);
        }

        let next = pos + dir;
        if limits.contains(&next) && grid[next as usize] != '#' {
            let mut path = path.clone();
            path.push(pos);
            priority_queue.push((priority - 1, next, dir, path));
        }

        for dir in {
            if dir == 1 || dir == -1 {
                [w, -w]
            } else {
                [1, -1]
            }
        } {
            priority_queue.push((priority - 1000, pos, dir, path.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
