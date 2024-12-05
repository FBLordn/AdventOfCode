use intmap::IntMap;

use super::{impl_day, Part1, Part2};

impl_day!(Day5, 5);

impl Part1 for Day5 {
    fn part1(&self, input: &str) -> usize {
        let mut map: IntMap<Vec<i32>> = IntMap::new();
        let t: Vec<&str> = input.split("\n\n").collect();
        let rl = t[0].lines().map(|f| {
            f.split("|")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });
        for i in rl {
            if map.contains_key(i[1] as u64) {
                let p = map.get(i[1] as u64).unwrap();
                map.insert(i[1] as u64, [p.to_owned(), vec![i[0]]].concat());
            } else {
                map.insert(i[1] as u64, vec![i[0]]);
            }
        }
        t[1].lines()
            .map(|f| {
                f.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .fold(0, |acc, x| {
                acc + if valid(map.clone(), x.clone()) {
                    x.get(x.len() / 2).unwrap()
                } else {
                    &0
                }
            }) as usize
    }
}

fn valid(map: IntMap<Vec<i32>>, line: Vec<i32>) -> bool {
    let mut occ: Vec<i32> = Vec::new();
    for d in line.clone() {
        occ.push(d);
        if map.contains_key(d as u64)
            && map
                .get(d as u64)
                .unwrap()
                .iter()
                .any(|f| !occ.contains(f) && line.contains(f))
        {
            return false;
        }
    }
    true
}

impl Part2 for Day5 {
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
            Day5.part1(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day5.part2(todo!()), todo!());
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day5.get_input();
        b.iter(|| Day5.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day5.get_input();
        b.iter(|| Day5.part2(&input));
    }
}
