#![feature(iter_next_chunk)]
advent_of_code::solution!(9);

fn get_numbers(text: &str) -> Vec<i64> {
    text.split(' ')
        .filter_map(|c| c.parse::<i64>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut result = 0;
    for line in input.lines() {
        let story = get_numbers(line);
        let mut current_line = story.clone();
        let mut story_lines = vec![story];
        let mut new_lines = vec![];
        while current_line.iter().any(|s| s != &0) {
            for (i, num) in current_line.iter().enumerate() {
                match current_line.get(i + 1) {
                    Some(s) => new_lines.push(s - num),
                    _ => {}
                }
            }
            current_line = new_lines;
            story_lines.push(current_line.clone());
            new_lines = vec![];
        }
        let mut next_inc = 0;
        for story in story_lines.iter_mut().rev() {
            let inc = story.get(story.len() - 1).unwrap() + next_inc;
            story.push(inc);
            next_inc = inc;
        }
        result += next_inc;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result = 0;
    for line in input.lines() {
        let story = get_numbers(line);
        let mut current_line = story.clone();
        let mut story_lines = vec![story];
        let mut new_lines = vec![];
        while current_line.iter().any(|s| s != &0) {
            for (i, num) in current_line.iter().enumerate() {
                match current_line.get(i + 1) {
                    Some(s) => new_lines.push(s - num),
                    _ => {}
                }
            }
            current_line = new_lines;
            story_lines.push(current_line.clone());
            new_lines = vec![];
        }
        let mut next_inc = 0;
        for story in story_lines.iter_mut().rev() {
            let inc = story.get(0).unwrap() - next_inc;
            story.push(inc);
            next_inc = inc;
        }
        result += next_inc;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
