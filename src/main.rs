#![feature(try_from)]
use std::env;

extern crate image;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate ndarray;
extern crate regex;

#[macro_use]
mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day16;
mod day17;
mod day18;

fn notimplemented(_: &[&str]) {
    println!("not implemented");
}

fn main() {
    let mut args = env::args();
    let binary = args.next().unwrap();
    let name = args.next();
    let tail = args.collect::<Vec<_>>();
    let rest = tail.iter().map(|x| x as &str).collect::<Vec<_>>();

    let funs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        notimplemented,
        day16::run,
        day17::run,
        day18::run,
        notimplemented,
        notimplemented,
        notimplemented,
        notimplemented,
        notimplemented,
        notimplemented,
        notimplemented,
    ];

    match name.clone().map(|x| x.parse::<usize>()) {
        Some(Ok(i)) if (i > 0 && i <= funs.len()) => {
            funs[i - 1](&rest);
        }
        Some(_) => {
            println!("invalid day: {:?}", name.unwrap_or_default());
            return;
        }
        _ => {
            println!("usage: {} [day]", binary);
            return;
        }
    }
}
