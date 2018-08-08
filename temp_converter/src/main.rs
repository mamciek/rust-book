#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::env;
use std::fmt;

enum TempUnit {
    C,
    F,
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TempUnit::C => write!(f, "C"),
            TempUnit::F => write!(f, "F"),
        }
    }
}

fn main() {
    for argument in env::args().skip(1) {
        let (value, unit) = parse_input(&argument);
        let (c_value, c_unit) = convert(value, &unit);
        println!("{}{} -> {}{}", value, unit, c_value, c_unit);
    }
}

fn parse_input(input: &str) -> (u32, TempUnit) {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(?i)(\d+)(C|F)").unwrap();
    }
    let caps = PATTERN.captures(input).unwrap();
    let value: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let unit = caps.get(2).unwrap().as_str().to_uppercase();
    let unit = match unit.as_ref() {
        "C" => TempUnit::C,
        "F" => TempUnit::F,
        &_ => panic!("Unknown unit type"),
    };
    (value, unit)
}

fn convert(value: u32, unit: &TempUnit) -> (f64, TempUnit) {
    match unit {
        TempUnit::C => ((1.8 * value as f64) + 32.0, TempUnit::F),
        TempUnit::F => ((value as f64 - 32.0) / 1.8, TempUnit::C),
    }
}
