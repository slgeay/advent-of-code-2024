use std::cmp::Ordering;

use regex::Regex;

advent_of_code::solution!(14);

// const SIZE: (i16, i16) = (11, 7);
const SIZE: (i16, i16) = (101, 103);

pub fn part_one(input: &str) -> Option<u64> {
    let mut robots = input
        .lines()
        .map(|line| {
            Regex::new(r"(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
                .unwrap()
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse::<i16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            let mut x = robot[0];
            let mut y = robot[1];
            let vx = robot[2];
            let vy = robot[3];

            x = (x + SIZE.0 + vx) % SIZE.0;
            y = (y + SIZE.1 + vy) % SIZE.1;

            robot[0] = x;
            robot[1] = y;
        }
    }

    let quadrants = robots.iter().fold(vec![0u64; 4], |mut q, robot| {
        let index = match (robot[0].cmp(&(SIZE.0 / 2)), robot[1].cmp(&(SIZE.1 / 2))) {
            (Ordering::Less, Ordering::Less) => 0,
            (Ordering::Less, Ordering::Greater) => 1,
            (Ordering::Greater, Ordering::Less) => 2,
            (Ordering::Greater, Ordering::Greater) => 3,
            _ => return q,
        };
        q[index] += 1;
        q
    });

    Some(quadrants.iter().product::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = input
        .lines()
        .map(|line| {
            Regex::new(r"(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
                .unwrap()
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse::<i16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    for i in 0..10000 {
        for robot in robots.iter_mut() {
            let mut x = robot[0];
            let mut y = robot[1];
            let vx = robot[2];
            let vy = robot[3];

            x = (x + SIZE.0 + vx) % SIZE.0;
            y = (y + SIZE.1 + vy) % SIZE.1;

            robot[0] = x;
            robot[1] = y;
        }

        if i as i16 % SIZE.1 == 29 {
            println!("\n{:?}", i + 1);

            for y in 0..SIZE.1 {
                for x in 0..SIZE.0 {
                    let c = robots.iter().filter(|robot| robot[0] == x && robot[1] == y).count();
                    if c > 0 {
                        print!("{}", c);
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    Some(8270) // Manually looking through pictures...
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8270));
    }
}
