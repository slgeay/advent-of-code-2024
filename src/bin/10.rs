use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn solution<T>(input: &str) -> Option<u64>
where
    T: FromIterator<(usize, usize)> + IntoIterator<Item = (usize, usize)> + Clone,
{
    let w = input.find('\n').unwrap() + 1;

    let positions = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '.' | '\n' => None,
            _ => Some((c.to_digit(10).unwrap() as u8, (i % w, i / w))),
        })
        .fold(
            std::collections::HashMap::new(),
            |mut acc: std::collections::HashMap<u8, HashSet<_>>, (c, (x, y))| {
                acc.entry(c).or_default().insert((x, y));
                acc
            },
        );

    let mut reachable: std::collections::HashMap<(usize, usize), T> = std::collections::HashMap::new();
    positions[&9].iter().for_each(|&(x, y)| {
        reachable.insert((x, y), T::from_iter([(x, y)]));
    });

    for i in (0..=8).rev() {
        for &(x, y) in positions[&i].iter() {
            reachable.insert(
                (x, y),
                positions[&(i + 1)]
                    .iter()
                    .filter(|&&(nx, ny)| x.abs_diff(nx) + y.abs_diff(ny) == 1)
                    .flat_map(|&(nx, ny)| reachable[&(nx, ny)].clone().into_iter())
                    .collect(),
            );
        }
    }

    Some(positions[&0].iter().map(|&(x, y)| reachable[&(x, y)].clone().into_iter().count() as u64).sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    solution::<HashSet<_>>(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solution::<Vec<_>>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
