use std::collections::{HashMap, HashSet};

use num::Integer;

use super::{impl_day, Part1, Part2};

impl_day!(Day8, 8);

impl Part1 for Day8 {
    fn part1(&self, input: &str) -> usize {
        let lines: Vec<&str> = input.lines().collect();
        let mut antennas = HashMap::new();
        lines
            .iter()
            .enumerate()
            .fold(HashSet::new(), |mut acc, (i, f)| {
                f.chars()
                    .enumerate()
                    .filter(|l| l.1 != '.')
                    .for_each(|(x, c)| {
                        let g: i32 = i as i32;
                        let y: i32 = x as i32;
                        antennas
                            .entry(c)
                            .and_modify(|list: &mut Vec<(i32, i32)>| {
                                list.iter().for_each(|ant| {
                                    acc.insert((g - (ant.0 - g), y - (ant.1 - y)));
                                    acc.insert((g + 2 * (ant.0 - g), y + 2 * (ant.1 - y)));
                                });
                                list.push((g, y));
                            })
                            .or_insert(vec![(g, y)]);
                    });
                acc
            })
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
        let lines: Vec<&str> = input.lines().collect();
        let mut antennas = HashMap::new();
        lines
            .iter()
            .enumerate()
            .fold(HashSet::new(), |mut acc, (i, f)| {
                f.chars()
                    .enumerate()
                    .filter(|l| l.1 != '.')
                    .for_each(|(x, c)| {
                        let g: i32 = i as i32;
                        let y: i32 = x as i32;
                        antennas
                            .entry(c)
                            .and_modify(|list: &mut Vec<(i32, i32)>| {
                                list.iter().for_each(|ant| {
                                    let gcd = (g - ant.0).gcd(&(y - ant.1));
                                    let dist = (((g - ant.0) / gcd), ((y - ant.1) / gcd));
                                    let mut i = 0;
                                    while g >= i * dist.0
                                        && g - i * dist.0 < lines.len() as i32
                                        && y >= i * dist.1
                                        && y - i * dist.1 < lines.first().unwrap().len() as i32
                                    {
                                        acc.insert((g - i * dist.0, y - i * dist.1));
                                        i += 1;
                                    }
                                    i = 1;
                                    while g + i * dist.0 < lines.len() as i32
                                        && g + i * dist.0 >= 0
                                        && y + i * dist.1 < lines.first().unwrap().len() as i32
                                        && y + i * dist.1 >= 0
                                    {
                                        acc.insert((g + i * dist.0, y + i * dist.1));
                                        i += 1;
                                    }
                                });
                                list.push((g, y));
                            })
                            .or_insert(vec![(g, y)]);
                    });
                acc
            })
            .len()
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
        assert_eq!(
            Day8.part2(
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
            34
        );
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
