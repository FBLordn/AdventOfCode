use std::vec;

use super::{impl_day, Part1, Part2};

impl_day!(Day1, 1);

impl Part1 for Day1 {
    fn part1(&self, input: &str) -> usize {
        let lines = input.lines();
        let mut first: Vec<usize> = vec![];
        let mut last: Vec<usize> = vec![];
        for i in lines {
            let t: Vec<&str> = i.split_whitespace().collect();
            first.push(t[0].parse::<usize>().unwrap());
            last.push(t[1].parse::<usize>().unwrap());
        }

        first.sort();
        first.iter().fold(0, |acc, f| {
            acc + last
                .remove(
                    last.iter()
                        .position(|x| x == last.iter().min().unwrap())
                        .unwrap(),
                )
                .abs_diff(*f)
        })
    }
}

impl Part2 for Day1 {
    fn part2(&self, input: &str) -> usize {
        let lines = input.lines();
        let mut first: Vec<usize> = vec![];
        let mut last: Vec<usize> = vec![];
        for i in lines {
            let t: Vec<&str> = i.split_whitespace().collect();
            first.push(t[0].parse::<usize>().unwrap());
            last.push(t[1].parse::<usize>().unwrap());
        }
        first.iter().fold(0, |acc, f| {
            acc + f * last.iter().filter(|n| n == &f).count()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use crate::day::DayMeta;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day1.part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day1.part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day1.get_input();
        b.iter(|| Day1.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day1.get_input();
        b.iter(|| Day1.part2(&input));
    }
}
