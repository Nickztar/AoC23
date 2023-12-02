advent_of_code::solution!(2);

struct Bag {
    red: i32,
    blue: i32,
    green: i32,
}

struct Game {
    name: u32,
    sets: Vec<Set>,
}

enum Set {
    Red(i32),
    Blue(i32),
    Green(i32),
}

impl Set {
    fn value(&self) -> &i32 {
        match self {
            Set::Green(s) => s,
            Set::Red(s) => s,
            Set::Blue(s) => s,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);
    Some(
        games
            .iter()
            .filter(|(id, game)| {
                let all_valid = game.iter().all(|g| {
                    let blue: i32 = g
                        .iter()
                        .filter(|g| matches!(g, Set::Blue(_)))
                        .map(|g| g.value())
                        .sum();
                    let green: i32 = g
                        .iter()
                        .filter(|g| matches!(g, Set::Green(_)))
                        .map(|g| g.value())
                        .sum();
                    let red: i32 = g
                        .iter()
                        .filter(|g| matches!(g, Set::Red(_)))
                        .map(|g| g.value())
                        .sum();
                    return blue <= 14 && green <= 13 && red <= 12;
                });
                all_valid
            })
            .map(|(id, _)| id)
            .sum::<i32>() as u32,
    )
}

fn parse_input(input: &str) -> Vec<(i32, Vec<Vec<Set>>)> {
    let lines = input.lines();
    let games = lines.into_iter().map(|line| {
        let (game, sets) = line.split_once(':').unwrap();
        let parsed = sets
            .split(';')
            .into_iter()
            .map(|set| {
                let parts = set
                    .split(',')
                    .into_iter()
                    .map(|part| {
                        let (count, color) = part.trim().split_once(' ').unwrap();
                        match color {
                            "red" => Set::Red(count.parse::<i32>().unwrap()),
                            "blue" => Set::Blue(count.parse::<i32>().unwrap()),
                            _ => Set::Green(count.parse::<i32>().unwrap_or(0)),
                        }
                    })
                    .collect();
                parts
            })
            .collect();
        (game.replace("Game ", "").parse::<i32>().unwrap(), parsed)
    });
    games.collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);
    Some(
        games
            .iter()
            .map(|(id, game)| {
                let colors: Vec<(i32, i32, i32)> = game
                    .iter()
                    .map(|g| {
                        let blue: i32 = g
                            .iter()
                            .filter(|g| matches!(g, Set::Blue(_)))
                            .map(|g| g.value())
                            .sum();
                        let green: i32 = g
                            .iter()
                            .filter(|g| matches!(g, Set::Green(_)))
                            .map(|g| g.value())
                            .sum();
                        let red: i32 = g
                            .iter()
                            .filter(|g| matches!(g, Set::Red(_)))
                            .map(|g| g.value())
                            .sum();
                        return (blue, green, red);
                    })
                    .collect();
                let maxB = colors.iter().map(|f| f.0).max().unwrap();
                let maxG = colors.iter().map(|f| f.1).max().unwrap();
                let maxR = colors.iter().map(|f| f.2).max().unwrap();
                maxB * maxG * maxR
            })
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
