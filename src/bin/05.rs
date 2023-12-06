#![feature(iter_array_chunks)]
use std::{collections::HashMap, io::SeekFrom};

advent_of_code::solution!(5);

fn get_numbers(text: &str) -> Vec<u64> {
    text.split(' ')
        .filter_map(|c| c.parse::<u64>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.replace("\r\n", "\n");
    let mut lines = input.split("\n\n").into_iter();
    let seeds = get_numbers(lines.next().unwrap());
    let mut current_seed_status = seeds.clone();

    for map in lines {
        let mut map_lines = map.lines();
        let _map_name = map_lines.next().unwrap();
        let ranges: Vec<(u64, u64, u64)> = map_lines
            .map(|s| {
                let numbers = get_numbers(s);
                let dest = numbers[0];
                let source = numbers[1];
                let length = numbers[2];
                (dest, source, length)
            })
            .collect();

        let new_seeds = current_seed_status
            .iter()
            .map(|seed| {
                let mut seed = *seed;
                // in range
                let target = &ranges
                    .iter()
                    .find(|(dest, source, length)| source <= &seed && &seed < &(source + length));
                if let Some((dest, source, length)) = target {
                    let diff = source + length - seed;
                    seed = dest + length - diff;
                }
                seed
            })
            .collect();
        current_seed_status = new_seeds;
    }

    Some(current_seed_status.into_iter().min().unwrap())
}

fn convert_range(map_range: &Vec<[u64; 3]>, seed_range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut ranges = vec![seed_range];
    let mut output = vec![];
    while let Some((start, end)) = ranges.pop() {
        if map_range.iter().all(|&[dst, src, len]| {
            let s = start.max(src);
            let e = end.min(src + len);
            if s < e {
                output.push((s - src + dst, e - src + dst));
                if start < s {
                    ranges.push((start, s));
                }
                if e < end {
                    ranges.push((e, end));
                }
                return false;
            }
            true // No intersection!
        }) {
            output.push((start, end));
        }
    }
    output
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.replace("\r\n", "\n");
    let mut lines = input.split("\n\n").into_iter();
    let seeds = get_numbers(lines.next().unwrap());
    let current_seed_status: Vec<_> = seeds.chunks(2).map(|c| (c[0], c[1])).collect();
    let mut mappers = Vec::new();
    let lines = lines.collect::<Vec<_>>();

    for map in lines.iter() {
        let ranges: Vec<[u64; 3]> = map
            .lines()
            .skip(1)
            .map(|s| {
                let numbers = get_numbers(s);
                let dest = numbers[0];
                let source = numbers[1];
                let length = numbers[2];
                [dest, source, length]
            })
            .collect();
        mappers.push(ranges);
    }

    let final_seed = current_seed_status
        .into_iter()
        .flat_map(|(seed, len)| {
            mappers.iter().fold(vec![(seed, seed + len)], |acc, map| {
                acc.into_iter()
                    .flat_map(|seed_range| convert_range(&map, seed_range))
                    .collect()
            })
        })
        .map(|(start, end)| {
            debug_assert!(start < end, "Invalid range");
            // start..end are all valid but we are looking for the minimum.
            start
        })
        .min();

    Some(final_seed.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
