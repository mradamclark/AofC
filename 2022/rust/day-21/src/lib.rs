use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{alpha1, line_ending, one_of},
    multi::separated_list1,
    sequence::{delimited, terminated},
    *,
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Job<'a> {
    Yell(f64),
    Math {
        lhs: &'a str,
        op: Operator,
        rhs: &'a str,
    },
}

#[derive(Debug, Copy, Clone)]
struct MonkeyJob<'a> {
    id: &'a str,
    job: Job<'a>,
}

fn operation(s: &str) -> IResult<&str, Job> {
    let (s, lhs) = alpha1(s)?;
    let (s, op) = delimited(
        tag(" "),
        one_of("+-/*").map(|c| match c {
            '*' => Operator::Mult,
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '/' => Operator::Div,
            _ => panic!("unknown Operator"),
        }),
        tag(" "),
    )(s)?;
    let (s, rhs) = alpha1(s)?;

    Ok((s, Job::Math { lhs, op, rhs }))
}

fn monkeyjob(s: &str) -> IResult<&str, MonkeyJob> {
    let (s, id) = terminated(alpha1, tag(": "))(s)?;
    let (s, job) = alt((complete::i64.map(|n| Job::Yell(n as f64)), operation))(s)?;

    Ok((s, MonkeyJob { id, job }))
}

fn parse(s: &str) -> IResult<&str, HashMap<&str, MonkeyJob>> {
    let (s, jobs) = separated_list1(line_ending, monkeyjob)(s)?;

    let jobs = jobs.into_iter().map(|job| (job.id, job)).collect();

    Ok((s, jobs))
}

fn do_monkey_math(job: &MonkeyJob, jobs: &HashMap<&str, MonkeyJob>) -> f64 {
    match job.job {
        Job::Yell(num) => num,
        Job::Math { lhs, op, rhs } => {
            let left = do_monkey_math(jobs.get(lhs).unwrap(), &jobs);
            let right = do_monkey_math(jobs.get(rhs).unwrap(), &jobs);
            match op {
                Operator::Mult => left * right,
                Operator::Div => left / right,
                Operator::Add => left + right,
                Operator::Sub => left - right,
            }
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, jobs) = parse(input).unwrap();
    let root = jobs.get("root").unwrap();
    do_monkey_math(root, &jobs).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut jobs) = parse(input).unwrap();

    //set human to yell 0, might be incorrect, but will cause a diff.
    //if 0 is ultimately correct, then we found the value.
    let mut yell: f64 = 0.0;
    jobs.insert(
        "humn",
        MonkeyJob {
            id: "humn",
            job: Job::Yell(yell),
        },
    );
    let root = jobs.get("root").unwrap().clone();
    let roots_jobs = jobs.clone();
    let (lhs, rhs) = match root.job {
        Job::Math { lhs, op: _, rhs } => {
            (roots_jobs.get(lhs).unwrap(), roots_jobs.get(rhs).unwrap())
        }
        _ => panic!("wtf"),
    };

    let humn_in_rhs = is_target_monkey_below(&jobs, rhs, "humn");

    if humn_in_rhs {
        let value = do_monkey_math(lhs, &jobs);
        // dbg!(value, rhs);
        yell = solve_for_humn(&jobs, rhs, value);
    } else {
        let value = do_monkey_math(rhs, &jobs);
        // dbg!(value, lhs);
        yell = solve_for_humn(&jobs, lhs, value);
    }
    yell.to_string()
}

fn solve_for_humn(jobs: &HashMap<&str, MonkeyJob>, curr: &MonkeyJob, value: f64) -> f64 {
    let job = curr.job;
    // dbg!(value, job);
    if let Job::Math { lhs, op, rhs } = job {
        let rhs = jobs.get(rhs).unwrap();
        let lhs = jobs.get(lhs).unwrap();

        let humn_in_rhs = is_target_monkey_below(jobs, rhs, "humn");
        // dbg!(humn_in_rhs);
        if humn_in_rhs {
            let mut lhs_num = do_monkey_math(lhs, &jobs);
            // dbg!(lhs_num);
            lhs_num = match op {
                Operator::Add => value - lhs_num,
                Operator::Sub => lhs_num - value,
                Operator::Mult => value / lhs_num,
                Operator::Div => value * lhs_num,
            };
            return solve_for_humn(&jobs, rhs, lhs_num);
        } else {
            let mut rhs_num = do_monkey_math(rhs, &jobs);
            // dbg!(rhs_num);
            rhs_num = match op {
                Operator::Add => value - rhs_num,
                Operator::Sub => value + rhs_num,
                Operator::Mult => value / rhs_num,
                Operator::Div => value * rhs_num,
            };
            return solve_for_humn(&jobs, lhs, rhs_num);
        }
    } else {
        if curr.id == "humn" {
            return value;
        } else {
            panic!("i don't know")
        }
    }
}

fn is_target_monkey_below(
    jobs: &HashMap<&str, MonkeyJob>,
    start: &MonkeyJob,
    target_id: &str,
) -> bool {
    match start.job {
        Job::Math { lhs, op: _, rhs } => {
            if lhs == target_id || rhs == target_id {
                true
            } else {
                let lhs = jobs.get(lhs).unwrap();
                if is_target_monkey_below(jobs, lhs, target_id) {
                    return true;
                }

                let rhs = jobs.get(rhs).unwrap();
                if is_target_monkey_below(jobs, rhs, target_id) {
                    return true;
                }
                false
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "152");
    }

    #[test]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "301");
    }
}
