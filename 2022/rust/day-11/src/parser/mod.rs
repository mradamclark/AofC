use std::collections::VecDeque;

use crate::{Monkey, Operation, Value};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    *,
};

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3

fn value(s: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::u64.map(|n| Value::Num(n)),
    ))(s)
}

fn operation(s: &str) -> IResult<&str, Operation> {
    let (s, _) = tag("Operation: new = ")(s)?;
    let (s, value_1) = value(s)?;
    let (s, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(s)?;
    let (s, value_2) = value(s)?;

    let result = match operator {
        "+" => Operation::Add((value_1, value_2)),
        "*" => Operation::Mult((value_1, value_2)),
        _ => panic!("Arg, unknown opcode"),
    };

    Ok((s, result))
}

fn test(s: &str) -> IResult<&str, Operation> {
    let (s, divisor) = preceded(tag("Test: divisible by "), complete::u64)(s)?;
    let test_operation = Operation::Div((Value::Curr, Value::Num(divisor)));
    Ok((s, test_operation))
}

fn starting_items(s: &str) -> IResult<&str, VecDeque<u64>> {
    let (s, _) = tag("Starting items: ")(s)?;
    let (s, items) = separated_list0(tag(", "), complete::u64)(s)?;
    Ok((s, VecDeque::from(items)))
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3

fn monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, id) = delimited(tag("Monkey "), complete::u32, tag(":"))(s)?;
    let (s, _) = multispace1(s)?;
    let (s, items) = starting_items(s)?;
    let (s, _) = multispace1(s)?;
    let (s, operation) = operation(s)?;
    let (s, _) = multispace1(s)?;
    let (s, test) = test(s)?;
    let (s, _) = multispace1(s)?;
    let (s, true_dest) = preceded(tag("If true: throw to monkey "), complete::u32)(s)?;
    let (s, _) = multispace1(s)?;
    let (s, false_dest) = preceded(tag("If false: throw to monkey "), complete::u32)(s)?;

    Ok((
        s,
        Monkey {
            id,
            items,
            operation,
            test,
            true_dest,
            false_dest,
            number_items_inspected: 0,
        },
    ))
}

pub fn parse(s: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey)(s)
}
