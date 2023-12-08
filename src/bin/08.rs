use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    let mappings = lines
        .skip(1)
        .map(|line| {
            let (start, directions) = line.split_once('=').unwrap();
            let (left, right) = directions.split_once(", ").unwrap();
            (
                start.trim(),
                (
                    left.trim().trim_start_matches('('),
                    right.trim_end_matches(')'),
                ),
            )
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut moves = 0;
    let mut current_step = "AAA";
    while current_step != "ZZZ" {
        let next_dir = directions.next().unwrap();
        let (l, r) = mappings.get(current_step).unwrap();
        current_step = match next_dir {
            'R' => r,
            _ => l,
        };
        moves += 1;
    }
    Some(moves)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().cycle();
    let mappings = lines
        .skip(1)
        .map(|line| {
            let (start, directions) = line.split_once('=').unwrap();
            let (left, right) = directions.split_once(", ").unwrap();
            (
                start.trim(),
                (
                    left.trim().trim_start_matches('('),
                    right.trim_end_matches(')'),
                ),
            )
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let starts: Vec<_> = mappings
        .iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(key) } else { None })
        .collect();
    let mut total_moves = vec![];
    dbg!(starts.len());
    for start in starts {
        let mut directions = directions.clone();
        let mut current_step = start;
        let mut moves = 0;
        while !current_step.ends_with('Z') {
            let next_dir = directions.next().unwrap();
            let (l, r) = mappings.get(current_step).unwrap();
            current_step = match next_dir {
                'R' => r,
                _ => l,
            };
            moves += 1;
        }
        total_moves.push(moves);
        dbg!(total_moves.len());
    }
    dbg!(&total_moves);
    let lcm = total_moves.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
    Some(lcm)
}
fn gcd(a: u64, b: u64) -> u64 {
    // Euclidean algorithm
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
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
        let p2 = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let result = part_two(p2);
        assert_eq!(result, Some(6));
    }
}
