advent_of_code::solution!(6);

fn get_numbers(text: &str) -> Vec<u64> {
    text.split(' ')
        .filter_map(|c| c.parse::<u64>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = get_numbers(lines.next().unwrap());
    let record = get_numbers(lines.next().unwrap());
    let total_wins: usize = (0..time.len())
        .map(|game| {
            let total = time[game];
            let target = record[game];
            (1..total)
                .filter(|ms| {
                    let rem = total - ms;
                    rem * ms > target
                })
                .count()
        })
        .product();
    Some(total_wins as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = get_numbers(
        &lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .replace(" ", ""),
    );
    let record = get_numbers(
        &lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(" ", ""),
    );
    let total_wins: usize = (0..time.len())
        .map(|game| {
            let total = time[game];
            let target = record[game];
            (1..total)
                .filter(|ms| {
                    let rem = total - ms;
                    rem * ms > target
                })
                .count()
        })
        .product();
    Some(total_wins as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
