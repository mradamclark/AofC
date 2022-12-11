pub mod handheld;
pub mod parser;

pub fn process_part1(input: &str) -> String {
    "todo".to_string()
}

pub fn process_part2(input: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1_works() {
        let input = fs::read_to_string("data/test.txt").unwrap();
        let result = crate::process_part1(&input);
        assert_eq!(result, "someanswer");
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./data/test.txt").unwrap();
        let result = crate::process_part2(&input);
        assert_eq!(result, "someanswer");
    }
}
