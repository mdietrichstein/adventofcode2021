fn main() {
    let input = include_str!("../resources/input");
    let num_paths = count_paths(input);
    println!("[1/2] Result: {}", num_paths);
}

fn count_paths(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::count_paths;

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
        assert_eq!(10, count_paths(TEST_DATA_SMALL));
        assert_eq!(19, count_paths(TEST_DATA_MEDIUM));
        assert_eq!(226, count_paths(TEST_DATA_LARGE));
    }

    #[test]
    pub fn test_part2() {}
}