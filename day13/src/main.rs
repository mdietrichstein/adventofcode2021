use map::Map;

mod map;

fn main() {
    let input = include_str!("../resources/input");

    let (map, folds) = parse_instructions(input).unwrap();
    let map = fold(map, folds, true);
    let num_non_empty = map.data().iter().filter(|value| **value > 0).count();
    println!("[1/2] Result: {}", num_non_empty);

    let (map, folds) = parse_instructions(input).unwrap();
    let map = fold(map, folds, false);
    println!("[2/2] Result:\n{}", map);
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

fn parse_instructions(input: &str) -> Option<(Map, Vec<(Axis, usize)>)> {
    let (coordinates, folds) = input.trim().split_once("\n\n")?;

    let coordinates: Vec<(usize, usize)> = coordinates.split('\n')
        .map(|entry| entry.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let num_rows = *coordinates.iter().map(|(x, y)| y).max().unwrap() + 1;
    let num_columns = *coordinates.iter().map(|(x, y)| x).max().unwrap() + 1;

    let mut map = Map::new_with_default_value(0, num_rows, num_columns);

    for (x, y) in coordinates {
        map[(y, x)] = 1;
    }

    let folds: Vec<(Axis, usize)> = folds.split('\n').into_iter()
        .map(|entry| entry.replace("fold along ", ""))
        .map(|entry| {
            let (axis, location) = entry.split_once('=').unwrap();
            (axis.trim().to_string(), location.parse::<usize>().unwrap())
        })
        .map(|(axis, location)| {
            if axis == "x" {
                (Axis::X, location)
            } else if axis == "y" {
                (Axis::Y, location)
            } else {
                panic!("Invalid instruction ({}, {})", axis, location)
            }
        }).collect();

    Some((map, folds))
}

fn fold(map: Map, folds: Vec<(Axis, usize)>, single_fold: bool) -> Map {
    let mut map = map;

    for (axis, location) in folds {
        match axis {
            Axis::X => {
                for x in 0..location {
                    let a = (location - 1 - x);
                    let b = (location + 1 + x);

                    for y in 0..map.num_rows() {
                        map[(y, a)] += map[(y, b)];
                    }
                }


                map = map.sliced_map(0, 0, map.num_rows(), location);

                if single_fold {
                    return map;
                }
            }
            Axis::Y => {
                for y in 0..location {
                    let a = (location - 1 - y);
                    let b = (location + 1 + y);

                    for x in 0..map.num_columns() {
                        if b < map.num_rows() {
                            map[(a, x)] += map[(b, x)];
                        }
                    }
                }

                map = map.sliced_map(0, 0, location, map.num_columns());

                if single_fold {
                    return map;
                }
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    ";

    #[test]
    fn test_part1() {
        let (map, folds) = parse_instructions(TEST_DATA).unwrap();
        let map = fold(map, folds, true);
        let num_non_empty = map.data().iter().filter(|value| **value > 0).count();
        assert_eq!(17, num_non_empty);
    }

    #[test]
    fn test_part2() {
        let (map, folds) = parse_instructions(TEST_DATA).unwrap();
        let map = fold(map, folds, false);
        let num_non_empty = map.data().iter().filter(|value| **value > 0).count();
        println!("{}", map);
        assert_eq!(16, num_non_empty);

    }
}