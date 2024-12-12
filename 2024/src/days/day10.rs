use super::{impl_day, Part1, Part2};

impl_day!(Day10, 10);

impl Part1 for Day10 {
    fn part1(&self, input: &str) -> usize {
        todo!()
    }
}

impl Part2 for Day10 {
    fn part2(&self, input: &str) -> usize {
        todo!()
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
        assert_eq!(Day10.part1(todo!()), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day10.part2(todo!()), todo!());
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day10.get_input();
        b.iter(|| Day10.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day10.get_input();
        b.iter(|| Day10.part2(&input));
    }
}