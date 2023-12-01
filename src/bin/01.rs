advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut acc: u32 = 0;
    for line in input.lines() {
        let digits = get_digits(line);
        match (digits.first(), digits.last()) {
            (Some(s), Some(l)) => acc += s * 10 + l,
            _ => {}
        }
    }

    Some(acc)
}

fn get_digits(l: &str) -> Vec<u32> {
    l.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn get_digits_2(l: &str) -> Vec<u32> {
    let l = l
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    get_digits(&l)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut acc: u32 = 0;
    for line in input.lines() {
        let digits = get_digits_2(line);
        match (digits.first(), digits.last()) {
            (Some(s), Some(l)) => acc += s * 10 + l,
            _ => {}
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
