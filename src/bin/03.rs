use regex::Regex;

advent_of_code::solution!(3);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|caps| {
                let (_full, [n1, n2]) = caps.extract();
                (n1, n2)
            })
            .map(|(n1, n2)| (n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()))
            .map(|(n1, n2)| n1 * n2)
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    // let x = parse_3(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, todo!());
    }
}
