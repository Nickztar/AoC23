use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

fn get_num(nums: &Vec<(i32, i32, char)>) -> u32 {
    let str = nums.iter().map(|s| s.2).collect::<String>();
    str.parse::<_>().unwrap()
}

fn store_num(nums: &Vec<(i32, i32, char)>, v: u32, map: &mut HashMap<(i32, i32), u32>) {
    for (y, x, _) in nums {
        map.insert((*y, *x), v);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut acc = vec![];
    let lookup = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(k, ch)| ((i as i32, k as i32), ch))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i32, i32), char>>();
    for (i, line) in input.lines().enumerate() {
        let mut curr_nums = vec![];
        for (k, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                curr_nums.push((i as i32, k as i32, ch));
            }
            assert!(curr_nums.len() <= 3);
            if !curr_nums.is_empty() {
                // Check if next on line is invalid char?
                if let Some(next_ch) = lookup.get(&(i as i32, k as i32 + 1)) {
                    if next_ch.is_digit(10) {
                        continue; //we handle it on the next check
                    } else if next_ch != &'.' {
                        // there is a char and that char is not a '.'
                        // convert to numbers?
                        let value = get_num(&curr_nums);
                        acc.push(value);
                        curr_nums.clear();
                        continue;
                    }
                }

                let mut any = false;
                for (n_y, n_x, _) in curr_nums.iter() {
                    // Check all the parts of it
                    let to_check: [(i32, i32); 8] = [
                        (-1, 0),  // straight up
                        (-1, -1), // up left
                        (0, -1),  // left
                        (1, -1),  // down left
                        (1, 0),   // down
                        (1, 1),   // down right
                        (0, 1),   // right
                        (-1, 1),  // up right
                    ];
                    for (y_diff, x_diff) in to_check {
                        match lookup.get(&(n_y + y_diff, n_x + x_diff)) {
                            Some(ch) => {
                                if !ch.is_digit(10) && ch != &'.' {
                                    any = true;
                                }
                            }
                            None => {}
                        }
                    }
                }
                if any {
                    acc.push(get_num(&curr_nums));
                }
                curr_nums.clear();
            }
        }
    }
    Some(acc.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let to_check: [(i32, i32); 8] = [
        (-1, 0),  // straight up
        (-1, -1), // up left
        (0, -1),  // left
        (1, -1),  // down left
        (1, 0),   // down
        (1, 1),   // down right
        (0, 1),   // right
        (-1, 1),  // up right
    ];
    let mut acc = vec![];
    let lookup = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(k, ch)| ((i as i32, k as i32), ch))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i32, i32), char>>();
    let mut number_lookup: HashMap<(i32, i32), u32> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut curr_nums = vec![];
        for (k, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                curr_nums.push((i as i32, k as i32, ch));
            }
            assert!(curr_nums.len() <= 3);
            if !curr_nums.is_empty() {
                // Check if next on line is invalid char?
                if let Some(next_ch) = lookup.get(&(i as i32, k as i32 + 1)) {
                    if next_ch.is_digit(10) {
                        continue; //we handle it on the next check
                    } else if next_ch != &'.' {
                        // there is a char and that char is not a '.'
                        // convert to numbers?
                        let value = get_num(&curr_nums);
                        acc.push(value);
                        store_num(&curr_nums, value, &mut number_lookup);
                        curr_nums.clear();
                        continue;
                    }
                }

                let mut any = false;
                for (n_y, n_x, _) in curr_nums.iter() {
                    // Check all the parts of it
                    for (y_diff, x_diff) in to_check {
                        match lookup.get(&(n_y + y_diff, n_x + x_diff)) {
                            Some(ch) => {
                                if !ch.is_digit(10) && ch != &'.' {
                                    any = true;
                                }
                            }
                            None => {}
                        }
                    }
                }
                if any {
                    let val = get_num(&curr_nums);
                    acc.push(val);
                    store_num(&curr_nums, val, &mut number_lookup);
                }
                curr_nums.clear();
            }
        }
    }

    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        for (k, ch) in line.chars().enumerate() {
            if ch == '*' {
                // Check all the parts of it
                let numbers: HashSet<&u32> = to_check
                    .iter()
                    .filter_map(|(y_diff, x_diff)| {
                        let to_look = (i as i32 + y_diff, k as i32 + x_diff);
                        number_lookup.get(&to_look)
                    })
                    .collect();
                dbg!(&numbers);
                if numbers.len() == 2 {
                    sum += **numbers.iter().nth(0).unwrap() * **numbers.iter().nth(1).unwrap();
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835))
    }
}
