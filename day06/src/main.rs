fn main() {
    let input = include_str!("../resources/input");

    println!("[1/2] Result: {}", count_entries(input, 80));
    println!("[1/2] Result: {}", count_entries(input, 256));
}

fn count_entries(input: &str, num_days: u32) -> u64 {
    const NUM_STATES: usize = 9;

    let mut counts_per_timer = [0u64; NUM_STATES];

    let timers: Vec<usize> = input.trim().split(',').map(|timer| timer.parse::<usize>().unwrap()).collect();

    for timer in timers {
        counts_per_timer[timer] += 1;
    }

    for _day in 0..num_days {
        let mut temp = [0u64; NUM_STATES];

        for i in 0..NUM_STATES - 1  {
            if i == 0 {
                temp[6] += counts_per_timer[0];
                temp[8] += counts_per_timer[0];
                temp[0] = counts_per_timer[i + 1];
            } else {
                temp[i] += counts_per_timer[i + 1];
            }
        }

        counts_per_timer = temp;
    }

    let num_entries: u64 = counts_per_timer.iter().sum();
    num_entries
}

#[cfg(test)]
mod test {
    use crate::count_entries;

    const TEST_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(26, count_entries(TEST_DATA, 18));
        assert_eq!(5934, count_entries(TEST_DATA, 80));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, count_entries(TEST_DATA, 256));
    }
}