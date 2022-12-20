use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{eof, iterator},
    sequence::terminated,
    IResult,
};
use tracing::{info, instrument};

fn parse(s: &str) -> IResult<&str, Vec<(usize, i64)>> {
    let mut it = iterator(s, terminated(complete::i64, alt((line_ending, eof))));
    let numbers = it.enumerate().collect::<Vec<_>>();

    let (input, _) = it.finish()?;
    Ok((input, numbers))
}

fn wrap_index(index: i64, size: i64) -> i64 {
    ((index % size) + size) % size
}

#[instrument]
pub fn process_part1(input: &str) -> String {
    let (_, mut numbers) = parse(input).unwrap();

    let length = numbers.len();

    for idx in 0..length {
        info!(
            "Before: {:?}",
            numbers.iter().map(|v| v.1).collect::<Vec<_>>()
        );

        let found_at_idx = numbers.iter().position(|(i, _)| i == &idx).unwrap();
        let item = numbers.remove(found_at_idx);

        info!(
            "After : {:?}",
            numbers.iter().map(|v| v.1).collect::<Vec<_>>()
        );

        info!("Fount at: {}", found_at_idx);
        let mut new_idx = found_at_idx as i64 + item.1;
        new_idx = wrap_index(new_idx, length as i64 - 1);

        info!("Inserting: {} at {}", item.1, new_idx);
        numbers.insert(new_idx as usize, item);
        info!("{:?}", numbers.iter().map(|v| v.1).collect::<Vec<_>>());
    }

    let zero_idx = numbers.iter().position(|v| v.1 == 0).unwrap();
    let one = numbers[(zero_idx + 1000) % length].1;
    let two = numbers[(zero_idx + 2000) % length].1;
    let three = numbers[(zero_idx + 3000) % length].1;
    info!(one, two, three, "Found Coords: ");
    let sum = one + two + three;
    sum.to_string()
}

const DKEY: i64 = 811589153;
pub fn process_part2(input: &str) -> String {
    let (_, mut numbers) = parse(input).unwrap();
    numbers.iter_mut().for_each(|tuple| tuple.1 *= DKEY);

    let length = numbers.len();
    for _ in 0..10 {
        for idx in 0..length {
            let found_at_idx = numbers.iter().position(|(i, _)| i == &idx).unwrap();
            let item = numbers.remove(found_at_idx);
            let mut new_idx = found_at_idx as i64 + item.1;
            new_idx = wrap_index(new_idx, length as i64 - 1);
            numbers.insert(new_idx as usize, item);
        }
    }

    let zero_idx = numbers.iter().position(|v| v.1 == 0).unwrap();
    let one = numbers[(zero_idx + 1000) % length].1;
    let two = numbers[(zero_idx + 2000) % length].1;
    let three = numbers[(zero_idx + 3000) % length].1;

    let sum = one + two + three;
    sum.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        tracing_subscriber::fmt::init();
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "1623178306");
    }
}
