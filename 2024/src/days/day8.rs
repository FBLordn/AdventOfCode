use std::collections::{HashMap, HashSet};

use super::{impl_day, Part1, Part2};

impl_day!(Day8, 8);

impl Part1 for Day8 {
    fn part1(&self, input: &str) -> usize {
        let lines: Vec<&str> = input.lines().collect();
        let mut antennas = HashMap::new();
        lines.iter().enumerate().for_each(|(i, f)| {
            f.chars()
                .enumerate()
                .filter(|l| l.1 != '.')
                .for_each(|(x, c)| {
                    antennas
                        .entry(c)
                        .and_modify(|list: &mut Vec<(i32, i32)>| {
                            list.push((i.try_into().unwrap(), x.try_into().unwrap()))
                        })
                        .or_insert(vec![(i.try_into().unwrap(), x.try_into().unwrap())]);
                });
        });
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        antennas.values().for_each(|f| {
            f.clone().iter().for_each(|g| {
                f.iter().filter(|h| g != *h).for_each(|t| {
                    antinodes.insert((g.0 - (t.0 - g.0), g.1 - (t.1 - g.1)));
                    antinodes.insert((g.0 + 2 * (t.0 - g.0), g.1 + 2 * (t.1 - g.1)));
                })
            })
        });
        antinodes
            .iter()
            .filter(|c| {
                c.0 >= 0
                    && c.0 < lines.len().try_into().unwrap()
                    && c.1 >= 0
                    && c.1 < lines.first().unwrap().len().try_into().unwrap()
            })
            .count()
    }
}

impl Part2 for Day8 {
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
        assert_eq!(
            Day8.part1(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            14
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8.part2(todo!()), todo!());
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| Day8.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| Day8.part2(&input));
    }
}
