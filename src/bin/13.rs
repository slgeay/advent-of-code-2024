use regex::Regex;

advent_of_code::solution!(13);

#[rustfmt::skip]
pub fn part_one(input: &str) -> Option<u64> {
    Some(input.split("\n\n").map(|machine|{Regex::new(r"(?s)(\d+).+\+(\d+).+\+(\d+).+\+(\d+).+=(\d+).+=(\d+)").unwrap().captures(machine).unwrap().iter().skip(1).map(|n|n.unwrap().as_str().parse::<f64>().unwrap()).collect::<Vec<_>>()}).map(|m|{let b=(m[5]*m[0]-m[4]*m[1])/(m[0]*m[3]-m[2]*m[1]);if b.fract()==0.0{let a=(m[4]-b*m[2])/m[0];return if a.fract()==0.0{(3.0*a+b)as u64}else{0}}0}).sum())
}

const D: f64 = 10000000000000.0;

#[rustfmt::skip]
pub fn part_two(input: &str) -> Option<u64> {
    Some(input.split("\n\n").map(|machine|{Regex::new(r"(?s)(\d+).+\+(\d+).+\+(\d+).+\+(\d+).+=(\d+).+=(\d+)").unwrap().captures(machine).unwrap().iter().skip(1).map(|n|n.unwrap().as_str().parse::<f64>().unwrap()).collect::<Vec<_>>()}).map(|m|{let b=((D+m[5])*m[0]-(D+m[4])*m[1])/(m[0]*m[3]-m[2]*m[1]);if b.fract()==0.0{let a=((D+m[4])-b*m[2])/m[0];return if a.fract()==0.0{(3.0*a+b)as u64}else{0}}0}).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
