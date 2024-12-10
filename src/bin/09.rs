advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    let (mut left_index, mut right_index) = (0, input.len() - 2);
    let (mut left_id, mut right_id) = (0, right_index / 2);
    let mut pos = 0;
    let mut right_length = input.chars().nth(right_index).unwrap().to_digit(10).unwrap();
    assert_ne!(right_length, 0);

    while left_index <= right_index {
        let left_length = input.chars().nth(left_index).unwrap().to_digit(10).unwrap();
        if left_index % 2 == 0 {
            assert_ne!(left_length, 0);
            for _ in 0..left_length {
                sum += pos * left_id;
                pos += 1;
                if left_id == right_id {
                    right_length -= 1;
                    if right_length == 0 {
                        break;
                    }
                }
            }
            left_id += 1;
        } else {
            for _ in 0..left_length {
                sum += pos * right_id;
                pos += 1;
                right_length -= 1;
                if right_length == 0 {
                    right_index -= 2;
                    right_id -= 1;
                    right_length = input.chars().nth(right_index).unwrap().to_digit(10).unwrap();
                    assert_ne!(right_length, 0);
                    if right_id < left_id {
                        break;
                    }
                }
            }
        }
        left_index += 1;
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memory: Vec<(u64, u64, i64)> = Vec::new();

    let merge_empty_space = |memory: &mut Vec<_>, slot: usize| {
        let (index, mut length, id) = memory[slot];
        assert_eq!(id, -1);
        let mut saved = (0, 0);
        if slot < memory.len() - 1 {
            let (next_index, next_length, next_id) = memory[slot + 1];
            assert_eq!(next_index, index + length);
            if next_id == -1 {
                length += next_length;
                memory[slot] = (index, length, -1);
                memory.remove(slot + 1);
                saved.1 = 1;
            }
        }
        if slot > 0 {
            let (prev_index, prev_length, prev_id) = memory[slot - 1];
            if prev_id == -1 {
                length += prev_length;
                memory[slot - 1] = (prev_index, length, -1);
                memory.remove(slot);
                saved.0 = 1;
            }
        }
        saved
    };

    let mut index = 0;
    let mut no_empty_slot_before = 0;
    for (slot, c) in input.chars().enumerate() {
        let length = c.to_digit(10).unwrap_or_default() as u64;
        if length > 0 {
            let is_empty = slot % 2 == 1;
            if is_empty && no_empty_slot_before == 0 {
                no_empty_slot_before = memory.len();
            }
            memory.push((index, length, if is_empty { -1 } else { slot as i64 / 2 }));
            index += length;
        }
    }

    let mut file_slot = memory.len() - 1;
    'outer: while file_slot > no_empty_slot_before {
        let (file_index, file_length, file_id) = memory[file_slot];
        if file_id != -1 {
            while memory[no_empty_slot_before].2 != -1 {
                no_empty_slot_before += 1;
                if file_slot <= no_empty_slot_before {
                    break 'outer;
                }
            }
            let mut empty_slot = no_empty_slot_before;
            while empty_slot < file_slot && (memory[empty_slot].2 != -1 || memory[empty_slot].1 < file_length) {
                empty_slot += 1;
            }
            if empty_slot < file_slot {
                let (empty_index, empty_length, empty_id) = memory[empty_slot];
                assert_eq!(empty_id, -1);
                memory[empty_slot] = (empty_index, file_length, file_id);
                if empty_length > file_length {
                    let unused_slot = empty_slot + 1;
                    memory.insert(unused_slot, (empty_index + file_length, empty_length - file_length, empty_id));
                    file_slot += 1;
                    let saved = merge_empty_space(&mut memory, unused_slot);
                    file_slot -= saved.0 + saved.1;
                }
                memory[file_slot] = (file_index, file_length, empty_id);
                let saved = merge_empty_space(&mut memory, file_slot);
                file_slot -= saved.0;
            }
        }
        file_slot -= 1;
    }

    Some(
        memory
            .iter()
            .map(|(index, length, id)| {
                if *id == -1 {
                    0
                } else {
                    (0..*length).map(|pos| *id as u64 * (index + pos)).sum()
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn brute_force(input: &str) -> Option<u64> {
        let input = &input[..input.len() - 1];
        let mut s = input
            .chars()
            .enumerate()
            .map(|(id, length)| vec![(if id % 2 == 0 { id as i64 / 2 } else { -1 })].repeat(length.to_digit(10).unwrap() as usize))
            .flatten()
            .collect::<Vec<i64>>();
        println!("{:?}", s);

        let mut sum = 0;
        let mut right = s.len() - 1;
        for left in 0..s.len() {
            if s[left] == -1 {
                while right > left && s[right] == -1 {
                    right -= 1;
                }
                s[left] = s[right];
                s[right] = -1;
            }
            if s[left] == -1 {
                break;
            }
            sum += s[left] * left as i64;
        }
        println!("{:?} => {}", s, sum);
        Some(sum as u64)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let mut input = format!("{}", rng.gen_range(1..10));
            for _ in 0..(rng.gen_range(1..100)) {
                input.push_str(&format!("{}{}", rng.gen_range(0..10), rng.gen_range(1..10)));
            }
            input.push_str("\n");
            println!("\n{}", input);
            let expected = brute_force(&input);
            let result = part_one(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
