advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut galaxy_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter_map(move |(j, g)| g.then_some((i, j)))
        })
        .collect();
    //Move galaxies on the x axis
    for i in (0..map.len()).rev() {
        if map[i].iter().all(|g| !g) {
            for (x, _) in &mut galaxy_positions {
                if *x > i {
                    *x += 1;
                }
            }
        }
    }

    //Move galaxies on the y axis
    for j in (0..map[0].len()).rev() {
        if (0..map.len()).map(|i| map[i][j]).all(|g| !g) {
            for (_, y) in &mut galaxy_positions {
                if *y > j {
                    *y += 1;
                }
            }
        }
    }

    let mut result = 0;

    for (i, a) in galaxy_positions.iter().enumerate() {
        for b in &galaxy_positions[(i + 1)..] {
            result += dist(a, b);
        }
    }

    Some(result)
}

//Simple position check
fn dist((a_x, a_y): &(usize, usize), (b_x, b_y): &(usize, usize)) -> usize {
    a_x.abs_diff(*b_x) + a_y.abs_diff(*b_y)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut galaxy_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter_map(move |(j, g)| g.then_some((i, j)))
        })
        .collect();
    //Move galaxies on the x axis
    for i in (0..map.len()).rev() {
        if map[i].iter().all(|g| !g) {
            for (x, _) in &mut galaxy_positions {
                if *x > i {
                    *x += 1_000_000 - 1;
                }
            }
        }
    }

    //Move galaxies on the y axis
    for j in (0..map[0].len()).rev() {
        if (0..map.len()).map(|i| map[i][j]).all(|g| !g) {
            for (_, y) in &mut galaxy_positions {
                if *y > j {
                    *y += 1_000_000 - 1;
                }
            }
        }
    }

    let mut result = 0;

    for (i, a) in galaxy_positions.iter().enumerate() {
        for b in &galaxy_positions[(i + 1)..] {
            result += dist(a, b);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
