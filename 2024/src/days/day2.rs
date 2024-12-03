use super::{impl_day, Part1, Part2};

impl_day!(Day2, 2);

impl Part1 for Day2 {
    fn part1(&self, input: &str) -> usize {
        let l = input
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()));
        l.map(|f| {
            let tup = f
                .clone()
                .map_windows(|[x, y]| {
                    if (1..=3).contains(&(x - y).abs()) {
                        if x < y {
                            (1, 0)
                        } else {
                            (0, 1)
                        }
                    } else {
                        (0, 0)
                    }
                })
                .fold((0, 0), |acc, f| (acc.0 + f.0, acc.1 + f.1));
            let bigger = if tup.0 < tup.1 { tup.1 } else { tup.0 };
            bigger == (f.count() - 1) as i32
            //.all(|n| (1..=3).contains(&(n - f.peek().unwrap_or(&(&n - 1)))))
        })
        .filter(|x| *x)
        .count()
    }
}

impl Part2 for Day2 {
    fn part2(&self, input: &str) -> usize {
        let l = input
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()));
        l.map(|f| {
            f.clone()
                .enumerate()
                .any(|n| valid_line(f.clone().enumerate().filter(|x| x.0 != n.0).map(|x| x.1)))
        })
        .filter(|x| *x)
        .count()
    }
}
fn valid_line(f: impl Iterator<Item = i32> + Clone) -> bool {
    let tup = f
        .clone()
        .map_windows(|[x, y]| {
            if (1..=3).contains(&(x - y).abs()) {
                if x < y {
                    (1, 0)
                } else {
                    (0, 1)
                }
            } else {
                (0, 0)
            }
        })
        .fold((0, 0), |acc, f| (acc.0 + f.0, acc.1 + f.1));
    let bigger = if tup.0 < tup.1 { tup.1 } else { tup.0 };
    bigger >= (f.count() - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day2.part1(
                "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
            ),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day2.part2(
                "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
            ),
            4
        );
    }
}
