advent_of_code::solution!(2);

pub(crate) fn parse_2(input: &str) -> impl Iterator<Item = Vec<(usize, i32)>> + '_ {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|s| s.map(|x| x.parse::<i32>().unwrap()).enumerate())
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
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Eq, Copy, Clone, Debug, Hash)]
enum Variant {
    Inc((usize, i32), (usize, i32)),
    Dec((usize, i32), (usize, i32)),
    Same(usize, usize, i32),
    InvalidInc((usize, i32), (usize, i32)),
    InvalidDec((usize, i32), (usize, i32)),
    Start,
}
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum VariantMarker {
    Inc,
    Dec,
    Same,
    InvalidInc,
    InvalidDec,
    Start,
}

impl Variant {
    fn marker(&self) -> VariantMarker {
        (*self).into()
    }
}

impl From<Variant> for VariantMarker {
    fn from(value: Variant) -> Self {
        match value {
            Variant::Inc(_, _) => Self::Inc,
            Variant::Dec(_, _) => Self::Dec,
            Variant::Same(_, _, _) => Self::Same,
            Variant::InvalidInc(_, _) => Self::InvalidInc,
            Variant::InvalidDec(_, _) => Self::InvalidDec,
            Variant::Start => Self::Start,
        }
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Inc(..), Self::Inc(..))
                | (Self::Dec(..), Self::Dec(..))
                | (Self::Same(..), Self::Same(..))
                | (Self::InvalidInc(..), Self::InvalidInc(..))
                | (Self::InvalidDec(..), Self::InvalidDec(..))
                | (Self::Start, Self::Start)
        )
    }
}

type NS = [(usize, i32)];

fn map_inc_dec(ns: &NS) -> impl Iterator<Item = Variant> + '_ {
    ns.windows(2).map(|x| {
        let &[(idx, a), (jdx, b)] = x else {
            unreachable!()
        };
        match b - a {
            0 => Variant::Same(idx, jdx, a),
            x if 0 < x && x < 4 => Variant::Inc((idx, a), (jdx, b)),
            x if -4 < x && x < 0 => Variant::Dec((idx, a), (jdx, b)),
            x if x.is_positive() => Variant::InvalidInc((idx, a), (jdx, b)),
            x if x.is_negative() => Variant::InvalidDec((idx, a), (jdx, b)),
            _ => unreachable!(),
        }
    })
}

fn safe_numbers_1(ns: &NS) -> bool {
    map_inc_dec(ns)
        .try_fold(Variant::Start, validate_pair)
        .is_some()
}

fn validate_pair(acc: Variant, curr: Variant) -> Option<Variant> {
    match (acc, curr) {
        (Variant::Start, x) => Some(x),
        (_, Variant::Same(..)) | (Variant::Same(..), _) => None,
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
#[allow(clippy::too_many_lines)]
fn safe_numbers_2(ns: &NS) -> bool {
    false
    // let ns_variant = map_inc_dec(ns).collect::<Vec<_>>();
    // let no_removal_required = ns_variant.windows(2).any(|it| {
    //     let &[a, b] = it else { unreachable!() };
    //     validate_pair(a, b).is_some()
    // });
    //
    // if no_removal_required {
    //     return true;
    // }
    //
    // let ans = (0..ns.len())
    //     .map(|idx| {
    //         ns[..idx]
    //             .iter()
    //             .copied()
    //             .chain(ns[(idx + 1)..].iter().copied())
    //             .collect::<Vec<_>>()
    //     })
    //     .any(|new_ns| {
    //         assert_eq!(ns.len(), new_ns.len() + 1);
    //         println!("{new_ns:?}");
    //         dbg!(safe_numbers_1(&new_ns))
    //     });
    // println!("{:-<16}", ans);
    //
    // ans
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Some(parse_2(input).filter(|nums| safe_numbers_2(nums)).count())
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
