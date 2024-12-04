use super::{impl_day, Part1, Part2};

use regex::Regex;

impl_day!(Day3, 3);

impl Part1 for Day3 {
    fn part1(&self, input: &str) -> usize {
        Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
            .unwrap()
            .find_iter(input)
            .fold(0, |acc, f| {
                acc + Regex::new(r"\d{1,3}")
                    .unwrap()
                    .find_iter(f.as_str())
                    .fold(1, |x, y| x * y.as_str().parse::<i32>().unwrap())
            }) as usize
    }
}

impl Part2 for Day3 {
    fn part2(&self, input: &str) -> usize {
        input.split("do()").fold(0, |sum, t| {
            sum + Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
                .unwrap()
                .find_iter(t.split("don't()").next().unwrap())
                .fold(0, |acc, f| {
                    acc + Regex::new(r"\d{1,3}")
                        .unwrap()
                        .find_iter(f.as_str())
                        .fold(1, |x, y| x * y.as_str().parse::<i32>().unwrap())
                })
        }) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day3.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day3.part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
