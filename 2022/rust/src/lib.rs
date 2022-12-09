#![allow(unused)]
use std::fmt::{Display, Formatter, Result};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub enum Part {
    One,
    Two,
}

macro_rules! output_from {
    ($(($name:tt, $datatype:ty)),*) => {
        #[derive(Debug)]
        pub enum Output {
            $($name($datatype)),*
        }

        $(
            impl From<$datatype> for Output {
                fn from(value: $datatype) -> Self {
                    Output::$name(value)
                }
            }
        )*
    };
}

output_from! {
    (U32,    u32),
    (String, String)
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Output::U32(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}
