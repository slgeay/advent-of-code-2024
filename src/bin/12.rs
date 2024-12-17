use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let w = input.find('\n').unwrap() + 1;
    let h = input.len() / w;

    let mut todo: HashSet<(usize, usize)> = (0..w - 1).flat_map(|x| (0..h).map(move |y| (x, y))).collect();

    let get_char = |x: usize, y: usize| input.as_bytes()[y * w + x];
    let mut regions: Vec<(usize, usize)> = Vec::new();

    while let Some(&(x_start, y_start)) = todo.iter().next() {
        let plant = input.as_bytes()[y_start * w + x_start];
        let mut region: (usize, usize) = (0, 0);

        todo.remove(&(x_start, y_start));

        let mut queue = VecDeque::new();
        queue.push_back((x_start, y_start));
        while let Some((x, y)) = queue.pop_front() {
            region.0 += 1;
            region.1 += 4;

            let neighbors = [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)];

            for &(nx, ny) in &neighbors {
                if nx < w - 1 && ny < h && get_char(nx, ny) == plant {
                    region.1 -= 1;
                    if todo.remove(&(nx, ny)) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        regions.push(region);
    }

    Some(regions.iter().map(|(a, p)| a * p).sum::<usize>() as u64)
}

const FENCE_RATIO: usize = 3;

pub fn part_two(input: &str) -> Option<u64> {
    let w = input.find('\n').unwrap() + 1;
    let h = input.len() / w;

    let mut todo: HashSet<(usize, usize)> = (0..w - 1).flat_map(|x| (0..h).map(move |y| (x, y))).collect();

    let get_char = |x: usize, y: usize| input.as_bytes()[y * w + x];
    let mut regions: Vec<(usize, HashSet<(usize, usize)>)> = Vec::new();

    while let Some(&(x_start, y_start)) = todo.iter().next() {
        let plant = input.as_bytes()[y_start * w + x_start];
        let mut region: (usize, HashSet<(usize, usize)>) = (0, HashSet::new());

        todo.remove(&(x_start, y_start));

        let mut queue = VecDeque::new();
        queue.push_back((x_start, y_start));
        while let Some((x, y)) = queue.pop_front() {
            let (x_fence, y_fence) = (FENCE_RATIO * x + 1, FENCE_RATIO * y + 1);
            region.0 += 1;
            for fence in &[(x_fence - 1, y_fence), (x_fence + 1, y_fence), (x_fence, y_fence - 1), (x_fence, y_fence + 1)] {
                region.1.insert(*fence);
            }

            for &(nx, ny) in &[(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)] {
                if nx < w - 1 && ny < h && get_char(nx, ny) == plant {
                    region.1.remove(&((x_fence + nx - x), (y_fence + ny - y)));
                    if todo.remove(&(nx, ny)) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        let mut fences = HashSet::new();
        while let Some(&fence) = region.1.iter().next() {
            region.1.remove(&fence);
            fences.insert(fence);

            let mut queue = VecDeque::new();
            queue.push_back(fence);
            while let Some((x_fence, y_fence)) = queue.pop_front() {
                for &(nx_fence, ny_fence) in &{
                    if x_fence % FENCE_RATIO == 1 {
                        [(x_fence.wrapping_sub(FENCE_RATIO), y_fence), (x_fence + FENCE_RATIO, y_fence)]
                    } else {
                        [(x_fence, y_fence.wrapping_sub(FENCE_RATIO)), (x_fence, y_fence + FENCE_RATIO)]
                    }
                } {
                    if region.1.remove(&(nx_fence, ny_fence)) {
                        queue.push_back((nx_fence, ny_fence));
                    }
                }
            }
        }
        region.1 = fences;
        regions.push(region);
    }

    Some(regions.iter().map(|(a, f)| a * f.len()).sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1184));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(368));
    }
}
