use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse(input: &str) -> impl IntoIterator<Item = (u32, u32)> + '_ {
    input.lines().map(|l| l.split_once("   ")).map(|maybe| {
        maybe
            .map(|(l, r)| {
                (
                    l.parse::<u32>().expect("failed to parse left"),
                    r.parse::<u32>().expect("failed to parse right"),
                )
            })
            .expect("empty line?")
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input).into_iter().collect::<(Vec<u32>, Vec<u32>)>();

    left.sort_unstable();
    right.sort_unstable();

    Some(
        left.into_iter()
            .zip(right)
            .fold(0, |diff, (left, right)| diff + left.abs_diff(right)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut occurrences = HashMap::new();
    let (left, right): (Vec<_>, Vec<_>) = parse(input).into_iter().unzip();

    for right in right {
        occurrences
            .entry(right)
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);
    }

    Some(
        left.into_iter()
            .map(|left| left * occurrences.get(&left).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
