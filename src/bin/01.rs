advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars();

            let first = it
                .find_map(|character| {
                    character.to_digit(10)
                })
                .expect("should be a number");

            let last = it
                .rfind(|character| {
                    character.is_ascii_digit()
                })
                .map(|character| {
                    character.to_digit(10).unwrap()
                })
                // if we don't find a number, then we're
                // re-using the first number
                .unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>();
    Some(output)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55834));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
