use std::vec;

use super::{impl_day, Part1, Part2};

impl_day!(Day4, 4);

impl Part1 for Day4 {
    fn part1(&self, input: &str) -> usize {
        let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        count_occurences(lines.iter().map(|f| f.iter()))
            + count_occurences(transformers_diag(input, false).iter().map(|f| f.iter()))
            + count_occurences(transformers_diag(input, true).iter().map(|f| f.iter()))
            + count_occurences(
                tranpose(lines.iter().map(|f| f.iter()))
                    .iter()
                    .map(|f| f.iter().copied()),
            )
    }
}

fn transformers_diag(input: &str, is_back: bool) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let max_l = lines[0].len() - 1;
    let mut ret: Vec<Vec<char>> = vec![vec![]; max_l + lines.len() + 1];
    for i in 0..lines.len() {
        for j in 0..=max_l {
            let t = if is_back { max_l - j } else { j };
            ret[i + t].push(lines[i][j]);
        }
    }
    ret
}

fn tranpose<T>(input: impl Iterator<Item: Iterator<Item = T>> + Clone) -> Vec<Vec<T>> {
    let len = input.clone().next().unwrap().count();
    let mut iters: Vec<_> = input.collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn count_occurences<'a>(input: impl Iterator<Item: Iterator<Item = &'a char>>) -> usize {
    input
        .into_iter()
        .map(|f| f.into_iter().collect::<String>())
        .fold(0, |sum, f| {
            sum + f.match_indices("XMAS").count() + f.match_indices("SAMX").count()
        })
}

impl Part2 for Day4 {
    fn part2(&self, input: &str) -> usize {
        let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut count = 0;
        for line in 1..lines.len() - 1 {
            for column in 1..lines[0].len() - 1 {
                if lines[line][column] == 'A'
                    && ((lines[line - 1][column - 1] == 'M' && lines[line + 1][column + 1] == 'S')
                        || (lines[line + 1][column + 1] == 'M'
                            && lines[line - 1][column - 1] == 'S'))
                    && ((lines[line - 1][column + 1] == 'M' && lines[line + 1][column - 1] == 'S')
                        || (lines[line + 1][column - 1] == 'M'
                            && lines[line - 1][column + 1] == 'S'))
                {
                    count += 1;
                }
            }
        }
        count
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
            Day4.part1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            18
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day4.part2(
                ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            ),
            9
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day4.get_input();
        b.iter(|| Day4.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day4.get_input();
        b.iter(|| Day4.part2(&input));
    }
}
