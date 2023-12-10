use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    // find S position
    // then do BFS from S
    // we must follow the shape logic:
    // J is _| shape,
    // F is |- shape,
    // L is L shape,
    // 7 is -| shape
    // we can go to the left if left char is F or L
    // we can go to the right if right char is J or 7
    // we can go up if up char is F or 7
    // we can go down if down char is J or L
    // also there are - and | chars

    // find S position
    let mut s_pos: (i32, i32) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos = (x as i32, y as i32);
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let steps = do_bfs(&mut map, s_pos, &mut visited);

    Some(steps)
}

fn do_bfs(map: &mut Vec<Vec<char>>, s_pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) -> i32 {
    // now do BFS from S
    let mut steps = 0;
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(s_pos);
    visited.insert(s_pos);
    while !queue.is_empty() {
        steps += 1;
        let sz = queue.len();
        for _ in 0..sz {
            let (x, y) = queue.remove(0);
            let curr_char = map[y as usize][x as usize];
            // check if we can go left
            if x > 0 {
                let left_char = map[y as usize][(x - 1) as usize];
                if (left_char == 'F' || left_char == 'L' || left_char == '-')
                    && (curr_char == 'S'
                        || curr_char == '7'
                        || curr_char == 'J'
                        || curr_char == '-')
                {
                    let left_pos = (x - 1, y);
                    if !visited.contains(&left_pos) {
                        queue.push(left_pos);
                        visited.insert(left_pos);
                    }
                }
            }
            // check if we can go right
            if x < map[0].len() as i32 - 1 {
                let right_char = map[y as usize][(x + 1) as usize];
                if (right_char == 'J' || right_char == '7' || right_char == '-')
                    && (curr_char == 'S'
                        || curr_char == 'F'
                        || curr_char == 'L'
                        || curr_char == '-')
                {
                    let right_pos = (x + 1, y);
                    if !visited.contains(&right_pos) {
                        queue.push(right_pos);
                        visited.insert(right_pos);
                    }
                }
            }
            // check if we can go up
            if y > 0 {
                let up_char = map[(y - 1) as usize][x as usize];
                if (up_char == 'F' || up_char == '7' || up_char == '|')
                    && (curr_char == 'S'
                        || curr_char == 'J'
                        || curr_char == 'L'
                        || curr_char == '|')
                {
                    let up_pos = (x, y - 1);
                    if !visited.contains(&up_pos) {
                        queue.push(up_pos);
                        visited.insert(up_pos);
                    }
                }
            }
            // check if we can go down
            if y < map.len() as i32 - 1 {
                let down_char = map[(y + 1) as usize][x as usize];
                if (down_char == 'J' || down_char == 'L' || down_char == '|')
                    && (curr_char == 'S'
                        || curr_char == 'F'
                        || curr_char == '7'
                        || curr_char == '|')
                {
                    let down_pos = (x, y + 1);
                    if !visited.contains(&down_pos) {
                        queue.push(down_pos);
                        visited.insert(down_pos);
                    }
                }
            }
        }
    }
    steps - 1
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    // find S position
    // then do BFS from S
    // we must follow the shape logic:
    // J is _| shape,
    // F is |- shape,
    // L is L shape,
    // 7 is -| shape
    // we can go to the left if left char is F or L
    // we can go to the right if right char is J or 7
    // we can go up if up char is F or 7
    // we can go down if down char is J or L
    // also there are - and | chars

    // find S position
    let mut s_pos: (i32, i32) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos = (x as i32, y as i32);
            }
        }
    }

    let mut pipes: HashSet<(i32, i32)> = HashSet::new();
    do_bfs(&mut map, s_pos, &mut pipes);

    // now we have all pipes that are in a loop
    // for each cell that is not in a pipe, we need to cast a ray
    // if it intersects with an odd number of lines, then the cell is in a loop

    let mut count_tiles_inside_loop = 0;
    for y in 0..map.len() - 0 {
        for x in 0..map[0].len() - 0 {
            // if it is not a pipe
            if pipes.contains(&(x as i32, y as i32)) {
                let char_in_pipe = match map[y][x] {
                    '-' => '─',
                    '|' => '│',
                    '7' => '┐',
                    'J' => '┘',
                    'F' => '┌',
                    'L' => '└',
                    'S' => ' ',

                    _ => panic!("unknown char "),
                };
                print!("{}", char_in_pipe);
            } else {
                if x == 0 || x == map[0].len() - 1 || y == 0 || y == map.len() - 1 {
                    print!(" ");
                    continue;
                }
                // cast a ray to the right
                let mut count_right = 0;
                for x2 in x + 1..map[0].len() {
                    let symbol = map[y][x2];
                    // this was the time when I gave up and searched for a solution
                    if (symbol == 'F' || symbol == '7' || symbol == '|')
                        && pipes.contains(&(x2 as i32, y as i32))
                    {
                        count_right += 1;
                    }
                }
                if count_right % 2 == 1 {
                    count_tiles_inside_loop += 1;
                }
                if count_right != 0 {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }

    Some(count_tiles_inside_loop)
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
        let ex = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part_two(ex);
        assert_eq!(result, Some(4));
    }
}
