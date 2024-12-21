use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let grid = parts.next().unwrap();
    let w = grid.find('\n').unwrap() as i16 + 1;
    let mut pos = grid.find('@').unwrap() as i16;
    let mut grid = grid.chars().collect::<Vec<_>>();
    let moves = parts.next().unwrap().chars();

    for m in moves {
        let dir = match m {
            '^' => -w,
            'v' => w,
            '<' => -1,
            '>' => 1,
            _ => continue,
        };

        let mut next_pos = pos + dir;
        let can_move_until = loop {
            match grid[next_pos as usize] {
                '#' => break pos,
                '.' => break next_pos,
                'O' => (),
                _ => unreachable!(),
            }
            next_pos += dir;
        };

        if can_move_until != pos {
            let mut empty = can_move_until;
            assert!(grid[empty as usize] == '.');
            while empty != pos {
                let next_empty = empty - dir;
                grid[empty as usize] = grid[next_empty as usize];
                empty = next_empty;
            }
            grid[pos as usize] = '.';
            pos += dir;
        }
    }

    let w = w as usize;

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, c)| match c {
                'O' => (100 * (i / w) + (i % w)) as u64,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let grid = parts.next().unwrap();
    let w = grid.find('\n').unwrap() as i16 * 2;
    let grid = grid
        .chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            '\n' => "",
            _ => unreachable!(),
        })
        .join("");
    let mut pos = grid.find('@').unwrap() as i16;
    let mut grid = grid.chars().collect::<Vec<_>>();
    let moves = parts.next().unwrap().chars();

    for m in moves {
        let dir = match m {
            '^' => -w,
            'v' => w,
            '<' => -1,
            '>' => 1,
            _ => continue,
        };

        if dir.abs() == 1 {
            let mut next_pos = pos + dir;
            let can_move_until = loop {
                match grid[next_pos as usize] {
                    '#' => break pos,
                    '.' => break next_pos,
                    '[' | ']' => (),
                    _ => unreachable!(),
                }
                next_pos += dir;
            };

            if can_move_until != pos {
                let mut empty = can_move_until;
                assert!(grid[empty as usize] == '.');
                while empty != pos {
                    let next_empty = empty - dir;
                    grid[empty as usize] = grid[next_empty as usize];
                    empty = next_empty;
                }
                grid[pos as usize] = '.';
                pos += dir;
            }
        } else {
            let mut visited = vec![false; grid.len()];
            visited[pos as usize] = true;
            let mut queue = VecDeque::new();
            queue.push_back((pos + dir, pos + dir));
            let can_move = loop {
                let (lookup_pos, next_pos) = queue.pop_front().unwrap();
                if visited[lookup_pos as usize] {
                    continue;
                }
                visited[lookup_pos as usize] = true;
                match grid[lookup_pos as usize] {
                    '#' => break false,
                    '.' => {
                        if queue.is_empty() {
                            break true;
                        }
                    }
                    '[' => {
                        queue.push_front((lookup_pos + 1, next_pos + dir));
                        queue.push_back((lookup_pos + dir, next_pos + dir));
                    }
                    ']' => {
                        queue.push_front((lookup_pos - 1, next_pos + dir));
                        queue.push_back((lookup_pos + dir, next_pos + dir));
                    }
                    _ => unreachable!(),
                };
            };

            if can_move {
                let iter: Box<dyn Iterator<Item = _>> = if dir > 0 {
                    Box::new(visited.iter().enumerate().rev())
                } else {
                    Box::new(visited.iter().enumerate())
                };
                for (pos, _) in iter.filter(|(_, v)| **v) {
                    let prev_pos = (pos as i16 - dir) as usize;
                    if visited[prev_pos] {
                        grid[pos] = grid[prev_pos];
                    } else {
                        grid[pos] = '.';
                    }
                }
                grid[pos as usize] = '.';
                pos += dir;
            }
        }
    }

    let w = w as usize;

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, c)| match c {
                '[' => (100 * (i / w) + (i % w)) as u64,
                _ => 0,
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
