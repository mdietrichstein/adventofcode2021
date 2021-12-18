fn main() {
    let input = include_str!("../resources/input");
    let velocities = find_velocities(input);

    println!("[1/2] Result: {}", find_max_height(&velocities));
    println!("[2/2] Result: {}", velocities.len());
}

fn parse_target_area(input: &str) -> ((i64, i64), (i64, i64)) {
    let parts = input.trim().split_ascii_whitespace().collect::<Vec<&str>>();
    let x = parts[2];
    let x = &x[2..x.len() - 1];

    let y = parts[3];
    let y = &y[2..y.len()];

    (str_to_range(x), str_to_range(y))
}

fn str_to_range(input: &str) -> (i64, i64) {
    input
        .split_once("..")
        .map(
            |(min, max)|
                (
                    min.parse::<i64>().unwrap(),
                    max.parse::<i64>().unwrap()
                )
        ).unwrap()
}

fn find_velocities(input: &str) -> Vec<(i64, i64)> {
    let target_area = parse_target_area(input);
    let ((xstart, xend), (ybottom, ytop)) = target_area;

    let min_vel_x = 1;
    let max_vel_x = xend;

    let min_vel_y = ybottom;
    let max_vel_y = -ybottom;

    let mut result: Vec<(i64, i64)> = vec![];

    for vel_x in min_vel_x..=max_vel_x {
        for vel_y in min_vel_y..=max_vel_y {
            let mut current_x = 0;
            let mut current_y = 0;

            let mut current_vel_x = vel_x;
            let mut current_vel_y = vel_y;

            loop {
                current_x += current_vel_x;
                current_y += current_vel_y;

                if current_x >= xstart && current_x <= xend && current_y <= ytop && current_y >= ybottom {
                    result.push((vel_x, vel_y));
                    break;
                }

                if current_x > xend || current_y < ybottom {
                    break;
                }

                if current_vel_x != 0 {
                    current_vel_x -= 1;
                }

                current_vel_y -= 1;
            }
        }
    }

    result
}

fn find_max_height(velocities: &Vec<(i64, i64)>) -> i64 {
    let max_vel_y = velocities.iter().map(|(_, y)| *y).max().unwrap();

    let max_height = if max_vel_y % 2 == 0 {
        (max_vel_y / 2) * (max_vel_y + 1)
    } else {
        ((max_vel_y + 1) / 2) * (max_vel_y)
    };

    max_height
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        let velocities = find_velocities(TEST_DATA);
        assert_eq!(45, find_max_height(&velocities));
    }

    #[test]
    fn test_part2() {
        let velocities = find_velocities(TEST_DATA);
        assert_eq!(112, velocities.len());
    }
}
