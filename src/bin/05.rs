use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(i: &str) -> Option<u32> {
    let (s, updates) = i.split("\n\n").collect_tuple()?;

    let mut rules: HashMap<_, Vec<_>> = HashMap::new();
    s.lines()
        .map(|r| r.split("|").map(|n| n.parse().unwrap()).collect_tuple().unwrap())
        .for_each(|(a, b)| rules.entry(a).or_insert_with(Vec::new).push(b));

    Some(
        updates
            .lines()
            .map(|update| {
                let mut seen = HashSet::new();
                let pages: Vec<_> = update.split(",").map(|s| s.parse().unwrap()).collect();
                pages
                    .iter()
                    .all(|&p| {
                        seen.insert(p);
                        !rules.get(&p).map_or(false, |r| r.iter().any(|r| seen.contains(r)))
                    })
                    .then(|| pages[pages.len() / 2])
                    .unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(i: &str) -> Option<u32> {
    let (s, updates) = i.split("\n\n").collect_tuple()?;

    let mut rules: HashMap<_, Vec<_>> = HashMap::new();
    s.lines()
        .map(|r| r.split("|").map(|n| n.parse().unwrap()).collect_tuple().unwrap())
        .for_each(|(a, b)| rules.entry(a).or_insert_with(Vec::new).push(b));

    Some(
        updates
            .lines()
            .map(|update| {
                let pages: Vec<_> = update.split(",").map(|s| s.parse().unwrap()).collect();
                let mut sorted_pages = pages.clone();
                sorted_pages.sort_by(|a, b| {
                    rules
                        .get(a)
                        .and_then(|r| r.contains(b).then_some(std::cmp::Ordering::Less))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                sorted_pages.ne(&pages).then(|| sorted_pages[pages.len() / 2]).unwrap_or(0)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
