use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(4);

fn get_numbers(cards: &str) -> Vec<u32> {
    cards
        .split(' ')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (_, game) = line.split_once(": ").unwrap();
        let (winning, losing) = game.split_once(" | ").unwrap();
        let winning = get_numbers(winning);
        let losing = get_numbers(losing);
        let my_win = losing
            .iter()
            .filter(|n| winning.contains(n))
            .enumerate()
            .collect::<Vec<_>>();
        let value = my_win.iter().fold(0, |acc, (i, _)| {
            acc + match i {
                0 => 1,
                _ => acc,
            }
        });
        total += value;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let input_lines = input.lines().collect::<Vec<&str>>();
    let mut lines = input.lines().collect::<VecDeque<&str>>();
    while let Some(line) = lines.pop_front() {
        let (n, game) = line.split_once(": ").unwrap();
        let card_value = n.split(' ').last().unwrap().parse::<usize>().unwrap();
        let (winning, losing) = game.split_once(" | ").unwrap();
        let winning = get_numbers(winning);
        let losing = get_numbers(losing);
        let my_win = losing
            .iter()
            .filter(|n| winning.contains(n))
            .enumerate()
            .collect::<Vec<_>>();
        let win_count = my_win.len();
        for n in 1..=win_count {
            lines.push_front(
                input_lines
                    .iter()
                    .nth((card_value - 1 + n) as usize)
                    .unwrap(),
            )
        }
        total += 1;
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
