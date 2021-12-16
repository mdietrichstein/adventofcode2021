use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use map::Map;

mod map;

fn main() {
    let input = include_str!("../resources/input");

    let cost = calculate_cost(Map::new(input));
    println!("[1/2] Result: {}", cost);

    let cost = calculate_cost(
        blow_up(Map::new(input), 5)
    );
    println!("[2/2] Result: {}", cost);
}

// See https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct Location {
    cost: usize,
    position: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_cost(map: Map) -> usize {
    // https://www.redblobgames.com/pathfinding/a-star/introduction.html
    let start = 0;
    let goal = map.len() - 1;

    let mut costs = HashMap::<usize, i64>::new();
    let mut came_from = HashMap::<usize, usize>::new();

    let mut frontier: BinaryHeap<Location> = BinaryHeap::new();
    frontier.push(Location {
        position: start,
        cost: 0,
    });

    costs.insert(start, 0);

    while !frontier.is_empty() {
        let location = frontier.pop().unwrap();
        let position = location.position;

        if position == goal {
            break;
        }

        let neighbors: Vec<Option<usize>> = map.neighborhood_positions(position).unwrap()
            .into_iter().enumerate().filter(|(i, position)| {
            // do not look at diagonal neighbors or self
            *i == 1 || *i == 3 || *i == 5 || *i == 7
        }).map(|(i, position)| position).collect();

        for next in neighbors {
            if let Some(index) = next {
                let new_cost = costs[&position] + map[index] as i64;

                if !costs.contains_key(&index) || new_cost < costs[&index] {
                    costs.insert(index, new_cost);

                    let priority = new_cost + map.manhattan_distance(goal, index).unwrap() as i64;
                    frontier.push(Location {
                        position: index,
                        cost: new_cost as usize,
                    });

                    came_from.insert(index, position);
                }
            }
        }
    }

    let mut path: Vec<usize> = vec![];
    path.push(goal);

    let mut current_position = goal;
    loop {
        let mut parent = came_from[&current_position];
        if parent == start {
            break;
        }

        path.push(parent);

        current_position = parent;
    }

    let cost = path.iter().map(|index| map[*index]).sum::<u64>();
    cost as usize
}

fn blow_up(map: Map, tile_size: usize) -> Map {
    let num_rows = map.num_rows() * tile_size;
    let num_columns = map.num_columns() * tile_size;

    let mut new_map = Map::new_with_default_value(0, num_rows, num_columns);

    for y in 0..num_rows {
        for x in 0..num_columns {
            let value = map[(y % map.num_rows(), x % map.num_columns())];
            let inc = (y / map.num_rows()) + (x / map.num_columns()) + value as usize;

            new_map[(y, x)] = if inc > 9 {
                inc - 9
            } else {
                inc
            } as u64;
        }
    }

    new_map
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    ";

    #[test]
    fn test_part1() {
        assert_eq!(40, calculate_cost(Map::new(TEST_DATA)));
    }

    #[test]
    fn test_part2() {
        let map = blow_up(Map::new(TEST_DATA), 5);
        assert_eq!(315, calculate_cost(map));
    }
}