use std::env;

extern crate regex;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    let mut args = env::args();
    let binary = args.next().unwrap();
    let name = args.next();
    let tail = args.collect::<Vec<_>>();
    let rest = tail.iter().map(|x| x as &str).collect::<Vec<_>>();

    let funs = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
        day11::run,
    ];

    match name.clone().map(|x| x.parse::<usize>()) {
        Some(Ok(i)) if (i > 0 && i <= funs.len()) => {
            funs[i - 1](&rest);
        },
        Some(_) => {
            println!("invalid day: {:?}", name.unwrap_or_default());
            return;
        },
        _ => {
            println!("usage: {} [day]", binary);
            return;
        }
    }
}
