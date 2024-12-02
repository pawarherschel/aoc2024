advent_of_code::solution!(2);

pub(crate) fn parse_2(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|s| s.map(|x| x.parse::<i32>().unwrap()))
        .map(Iterator::collect)
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, *2* reports are safe.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Variant {
    Inc,
    Dec,
    Same,
    Invalid,
    Start,
}

fn map_inc_dec(ns: &[i32]) -> impl Iterator<Item = Variant> + '_ {
    ns.windows(2).map(|x| {
        let &[a, b] = x else { unreachable!() };
        match a - b {
            0 => Variant::Same,
            x if 0 < x && x < 4 => Variant::Dec,
            x if -4 < x && x < 0 => Variant::Inc,
            _ => Variant::Invalid,
        }
    })
}

fn safe_numbers_1(ns: &[i32]) -> bool {
    map_inc_dec(ns)
        .try_fold(Variant::Start, validate_pair)
        .is_some()
}

fn validate_pair(acc: Variant, curr: Variant) -> Option<Variant> {
    match (acc, curr) {
        (Variant::Start, x) => Some(x),
        (_, Variant::Same) | (Variant::Same, _) => None,
        (x, y) if x == y => Some(x),
        _other => None,
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Some(parse_2(input).filter(|nums| safe_numbers_1(nums)).count())
}

//- `7 6 4 2 1`: *Safe* without removing any level.
// - `1 2 7 8 9`: *Unsafe* regardless of which level is removed.
// - `9 7 6 2 1`: *Unsafe* regardless of which level is removed.
// - `1 3 2 4 5`: *Safe* by removing the second level, `3`.
// - `8 6 4 4 1`: *Safe* by removing the third level, `4`.
// - `1 3 6 7 9`: *Safe* without removing any level.
//
// Thanks to the Problem Dampener, `4` reports are actually *safe*!

fn safe_numbers_2(ns: &[i32]) -> bool {
    todo!()
    // let mut x = map_inc_dec(ns).collect::<Vec<_>>();
    // let mut deviations = 0;
    //
    // let idx = x.windows(3).enumerate().find(|(idx, x)| {
    //     let &&[a, b, c] = x else { unreachable!() };
    //     (a == b && b != c) || (a != b && b == c) || (a == c && b != c)
    // });
    //
    // println!("idx: {idx:?}");
    //
    // if idx.is_none() {
    //     return true;
    // }
    //
    // let (idx, &[a, b, c]) = idx.unwrap() else {
    //     unreachable!()
    // };
    // println!("idx={idx}");
    //
    // let base = 3 * (idx.saturating_sub(1));
    // println!("base={base}");
    // let problematic_idx = match (a, b, c) {
    //     // c
    //     (a, b, c) if a == b && b != c => 2,
    //     // a
    //     (a, b, c) if a != b && b == c => 0,
    //     // b
    //     (a, b, c) if a == c && b != c => 1,
    //     _ => unreachable!(),
    // };
    // println!("problematic_idx={problematic_idx}");
    //
    // let idx = base + problematic_idx;
    // println!("idx={idx}");
    // let ns = ns[..idx]
    //     .iter()
    //     .chain(ns[(idx + 1)..].iter())
    //     .collect::<Vec<_>>();
    //
    // ns.windows(3).any(|x| {
    //     let &[a, b, c] = x else { unreachable!() };
    //     (a == b && b != c) || (a != b && b == c) || (a == c && b != c)
    // })

    // let &[a, b, c] = idx.unwrap();

    // for window in x.windows(3) {
    //     let &[a, b, c] = window else { unreachable!() };
    //     println!("{a:?} {b:?} {c:?}");
    //     match (a, b, c) {
    //         (a, b, c) if a == b && b == c => {}
    //         (a, b, c) if (a == b && b != c) || (a != b && b == c) | (a == c && b != c) => {
    //             deviations += 1
    //         }
    //         other => {
    //             println!("other: {other:?}")
    //         }
    //     }
    // }

    // deviations <= 2
    // x.windows(3)
    //     .fold((0, None), |(acc, prev), x| {
    //         let &[a, b, c] = x else { unreachable!() };
    //         println!("{a:?} {b:?} {c:?}");
    //         match (a, b, c) {
    //             (Variant::Start, a, b) => panic!("how?"),
    //             // `a` outlier
    //             (a, b, c) if a != b && b == c && b != Variant::Same => (acc + 1, Some(b)),
    //             // `b` outlier
    //             (a, b, c) if a != b && a == c && a != Variant::Same => (acc + 1, Some(a)),
    //             // `c` outlier
    //             (a, b, c) if a == b && b != c && a != Variant::Same => (acc + 1, Some(a)),
    //             (a, b, c) if a == b && b == c => (acc, Some(a)),
    //             others => {
    //                 dbg!(others);
    //                 (acc + 1, None)
    //             }
    //         }
    //     })
    //     .0
    //     <= 2
    // // .inspect(|x| println!("{x:?}"))
    // .filter(Option::is_none)
    // .count()
    // <= 1
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_2(input)
            .filter(|nums| {
                println!("{nums:?}");
                dbg!(safe_numbers_2(nums))
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
