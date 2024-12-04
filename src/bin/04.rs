use std::panic;

macro_rules! A {
    ($s:expr) => {{
        let h=panic::take_hook();
        panic::set_hook(Box::new(|_|{}));
        let r=panic::catch_unwind(||($s.collect::<String>()=="XMAS")as u32).unwrap_or(0);
        panic::set_hook(h);
        r
    }};
}

advent_of_code::solution!(4);

pub fn part_one(i: &str) -> Option<u32> {
    let l:Vec<Vec<char>>=i.lines().map(|s| s.chars().collect()).collect();
    Some((0..l.len()).fold(0,|a,i|{a+(0..l[i].len()).fold(0,|b,j|{b+A!(l[i][j..j+4].iter())
                + A!(l[i][j-3..j+1].iter().rev())
                + A!(l[i..i+4].iter().map(|l| l[j]))
                + A!(l[i-3..i+1].iter().rev().map(|l|l[j]))
                + A!(l[i..i+4].iter().enumerate().map(|(k,l)|l[j+k]))
                + A!(l[i..i+4].iter().enumerate().map(|(k,l)|l[j-k]))
                + A!(l[i-3..i+1].iter().rev().enumerate().map(|(k,l)|l[j+k]))
                + A!(l[i-3..i+1].iter().rev().enumerate().map(|(k,l)|l[j-k]))
        })
    }))
}

pub fn part_two(i: &str) -> Option<u32> {
    let l:Vec<Vec<char>>=i.lines().map(|s| s.chars().collect()).collect();
    Some((1..l.len()-1).map(|i|{(1..l[i].len()-1).map(|j|{[-1,1].map(|d|{matches!((0..3).map(|k|{l[i+k-1][(j as i32+(d*(k as i32-1)))as usize]}).collect::<String>().as_str(),"MAS"|"SAM")}).iter().all(|&x|x)as u32}).sum::<u32>()}).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
