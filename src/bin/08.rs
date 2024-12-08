use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let w = input.find('\n').unwrap() as i32 + 1;
    let h = input.len() as i32 / w;
    let dict = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '.' | '\n' => None,
            _ => Some((c, (i as i32 % w, i as i32 / w))),
        })
        .fold(HashMap::new(), |mut acc: HashMap<char, Vec<_>>, (c, (x, y))| {
            acc.entry(c).or_default().push((x, y));
            acc
        });

    let mut res = HashSet::new();
    dict.values().for_each(|v| {
        v.iter().enumerate().for_each(|(i, (x1, y1))| {
            v.iter().skip(i + 1).for_each(|(x2, y2)| {
                [(x1 * 2 - x2, y1 * 2 - y2), (x2 * 2 - x1, y2 * 2 - y1)]
                    .iter()
                    .filter(|(x, y)| (0..w - 1).contains(x) && (0..h).contains(y))
                    .for_each(|&a| {
                        res.insert(a);
                    });
            });
        });
    });
    Some(res.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = input.find('\n').unwrap() as i32 + 1;
    let h = input.len() as i32 / w;
    let dict = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '.' | '\n' => None,
            _ => Some((c, (i as i32 % w, i as i32 / w))),
        })
        .fold(HashMap::new(), |mut acc: HashMap<char, Vec<_>>, (c, (x, y))| {
            acc.entry(c).or_default().push((x, y));
            acc
        });

    let mut res = HashSet::new();
    dict.values().for_each(|v| {
        v.iter().enumerate().for_each(|(i, (x1, y1))| {
            v.iter().skip(i + 1).for_each(|(x2, y2)| {
                for (mut p, d) in [((*x1, *y1), (x1 - x2, y1 - y2)), ((*x2, *y2), (x2 - x1, y2 - y1))] {
                    while (0..w).contains(&p.0) && (0..h).contains(&p.1) {
                        res.insert(p);
                        p = (p.0 + d.0, p.1 + d.1);
                    }
                }

            });
        });
    });
    Some(res.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
