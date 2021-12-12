use map::Map;

mod map;

fn main() {
    let input = include_str!("../resources/input");

    let (_step, num_flashes) = count_flashes(input, false);
    println!("[1/2] Result: {}", num_flashes);

    let (step, _num_flashes) = count_flashes(input, true);
    println!("[2/2] Result: {}", step + 1);
}

fn count_flashes(input: &str, stop_when_all_flash: bool) -> (usize, usize) {
    let mut map = Map::new(input);
    let mut positions_to_check: Vec<usize> = vec![];

    let mut num_flashes: usize = 0;

    // println!("Before any steps:\n-------------------\n{}\n-------------------\n\n", map);

    let num_steps = if stop_when_all_flash {
        5000
    } else {
        100
    };

    for step in 0..num_steps {
        for i in 0..map.len() {
            map[i] += 1;

            if map[i] == 10 {
                positions_to_check.push(i);
            }
        }

        while let Some(position) = positions_to_check.pop() {
            let neighborhood_indices = map.neighborhood_positions(position);

            if let Some(neighborhood_indices) = neighborhood_indices {
                for index in neighborhood_indices {
                    if let Some(index) = index {
                        map[index] += 1;

                        if map[index] == 10 {
                            positions_to_check.push(index);
                        }
                    }
                }
            }
        }

        let mut num_step_flashes = 0;

        for i in 0..map.len() {
            if map[i] >= 10 {
                num_step_flashes += 1;
                map[i] = 0;
            }
        }

        num_flashes += num_step_flashes;

        if stop_when_all_flash && num_step_flashes == map.len() {
            println!("After step {}:\n-------------------\n{}\n-------------------\n\n", step + 1, map);
            return (step, num_flashes);
        }

        // println!("After step {}:\n-------------------\n{}\n-------------------\n\n", step + 1, map);
    }

    (num_steps - 1, num_flashes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";

    #[test]
    fn test_part1() {
        let (_step, num_flashes) = count_flashes(TEST_DATA, false);
        assert_eq!(1656, num_flashes);
    }

    #[test]
    fn test_part2() {
        let (step, _num_flashes) = count_flashes(TEST_DATA, true);
        assert_eq!(195, step + 1);
    }
}