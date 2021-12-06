use std::collections::HashMap;

fn main() {
    let input = include_str!("../resources/input");
    let lines = parse_lines(input);
    
    let intersections = find_intersections(&lines, false);
    println!("[1/2] Result: {}", intersections.len());

    let intersections = find_intersections(&lines, true);
    println!("[2/2] Result: {}", intersections.len());
}

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut parts = input.split_ascii_whitespace();
    
    let mut lines = vec![];

    loop {
        match (parts.next(), parts.next(), parts.next()) {
            (Some(start), Some(_), Some(end)) => {
                let mut start = start.split(',');
                let mut end = end.split(',');

                lines.push(
                    Line {
                        x1: start.next().unwrap().parse::<i32>().unwrap(),
                        y1: start.next().unwrap().parse::<i32>().unwrap(),
                        x2: end.next().unwrap().parse::<i32>().unwrap(),
                        y2: end.next().unwrap().parse::<i32>().unwrap()
                    }
                );
                
            },
            _ => break
        }
    }

    lines
}

fn find_intersections(lines: &Vec<Line>, allow_diagonals: bool) -> Vec<(i32, i32)> {
    let mut points: HashMap<(i32, i32), usize> = HashMap::new();

    for line in lines {
    
        if line.y1 == line.y2 {     // horizontal
            let (start, end) = if line.x1.min(line.x2) == line.x1 {
                (line.x1, line.x2)
            } else {
                (line.x2, line.x1)
            };

            for x in start..(end + 1) {
                *points.entry((x, line.y1)).or_insert(0) += 1;
            }
        } else if line.x1 == line.x2 {   // vertical
            let (start, end) = if line.y1.min(line.y2) == line.y1 {
                (line.y1, line.y2)
            } else {
                (line.y2, line.y1)
            };

            for y in start..(end + 1) {
                *points.entry((line.x1, y)).or_insert(0) += 1;
            }
        } else if allow_diagonals { // diagonal
            let (x_start, x_end) = if line.x1.min(line.x2) == line.x1 {
                (line.x1, line.x2)
            } else {
                (line.x2, line.x1)
            };

            let (y_start, y_end) = if line.x1 == x_start {
                (line.y1, line.y2)
            } else {
                (line.y2, line.y1)
            };

            let k = if (y_end - y_start) > 0 {
                1
            } else {
                -1
            };

            for (i, x) in (x_start..(x_end + 1)).enumerate() {
                *points.entry((x, y_start + (i as i32 * k))).or_insert(0) += 1;
            }
        }
    }

    let intersections: Vec<(i32, i32)> = points.into_iter()
        .filter_map(
            |(point, crossings)| 
                if crossings > 1 {
                    Some(point)
                } else {
                    None
                }
        )
        .collect();

    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA:&str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";

    #[test]
    fn test_part1() {
        let lines = parse_lines(TEST_DATA);
        let intersections = find_intersections(&lines, false);
        
        assert_eq!(5, intersections.len());
    }

    #[test]
    fn test_part2() {
        let lines = parse_lines(TEST_DATA);
        let intersections = find_intersections(&lines, true);
        
        assert_eq!(12, intersections.len());
    }
}