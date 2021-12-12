use std::collections::HashMap;
use std::collections::HashSet;

const START_NODE: &str = "start";
const END_NODE: &str = "end";

fn main() {
    let input = include_str!("../resources/input");
    let paths = find_paths(input);
    println!("[1/2] Result: {}", paths.len());
}

fn is_lowercase(value: &str) -> bool {
    value.to_lowercase() == value
}

fn visit_node<'a>(node: &'a str, mut visited: Vec<&'a str>, connections: &HashMap<&'a str, HashSet<&'a str>>, paths: &mut Vec<Vec<&'a str>>) {
    visited.push(node);

    for child in &connections[node] {
        if is_lowercase(child) && visited.contains(&child) {
            continue;
        }

        if *child == END_NODE {
            let mut path = visited.clone();
            path.push(child);
            paths.push(path);
            continue;
        }

        visit_node(child, visited.clone(), connections, paths);
    }
}

fn find_paths(input: &str) -> Vec<Vec<&str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.trim().split('\n')
        .map(|line| line.trim().split_once('-').unwrap())
        .for_each(|(a, b)| {
            let nodes = connections.entry(a).or_default();
            nodes.insert(b);

            let nodes = connections.entry(b).or_default();

            if a != START_NODE && b != END_NODE {
                nodes.insert(a);
            }
        });

    let mut paths: Vec<Vec<&str>> = vec![];

    let mut visited: Vec<&str> = vec![];
    visit_node(START_NODE, visited, &connections, &mut paths);

    paths
}


#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::find_paths;

    const TEST_DATA_SMALL: &str = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";

    const TEST_DATA_MEDIUM: &str = "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";

    const TEST_DATA_LARGE: &str = "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";

    #[test]
    pub fn test_part1() {
        find_paths(TEST_DATA_MEDIUM);
        assert_eq!(10, find_paths(TEST_DATA_SMALL).len());
        assert_eq!(19, find_paths(TEST_DATA_MEDIUM).len());
        assert_eq!(226, find_paths(TEST_DATA_LARGE).len());
    }

    #[test]
    pub fn test_part2() {}
}