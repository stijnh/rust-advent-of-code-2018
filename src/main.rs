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

fn main() {
    let mut args = env::args();
    let binary = args.next().unwrap();
    let name = args.next().unwrap_or("".to_string());
    let tail = args.collect::<Vec<_>>();
    let rest = tail.iter().map(|x| x as &str).collect::<Vec<_>>();

    if name == "1" {
        day1::run(&rest);
    } else if name == "2" {
        day2::run(&rest);
    } else if name == "3" {
        day3::run(&rest);
    } else if name == "4" {
        day4::run(&rest);
    } else if name == "5" {
        day5::run(&rest);
    } else if name == "6" {
        day6::run(&rest);
    } else if name == "7" {
        day7::run(&rest);
    } else if name == "" {
        println!("usage: {} [day]", binary);
    } else {
        println!("invalid day: {}", name);
    }
}
