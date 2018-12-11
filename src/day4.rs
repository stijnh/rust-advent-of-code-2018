use crate::common::read_file_lines;
use regex::Regex;
use std::collections::HashMap;

type Date = (i32, i32, i32, i32, i32);

#[derive(Debug)]
enum Event {
    WakeUp,
    FallAsleep,
    BeginShift(i32),
}

#[derive(Debug)]
struct Record {
    date: Date,
    event: Event,
}

pub fn run(_: &[&str]) {
    let re_line = Regex::new(r"\[(\d\d\d\d)[-](\d\d)[-](\d\d) (\d\d):(\d\d)\] (.*)").unwrap();
    let re_guard = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut records = vec![];

    for line in read_file_lines("inputs/day4") {
        let cap = re_line.captures(&line).unwrap();
        let mut matches = cap.iter().map(|x| x.unwrap()).map(|x| x.as_str()).skip(1);

        let date = matches
            .by_ref()
            .take(5)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let event = match matches.next().unwrap() {
            "falls asleep" => Event::FallAsleep,
            "wakes up" => Event::WakeUp,
            msg => {
                let matches = re_guard.captures(&msg).unwrap();
                let id = matches.get(1).unwrap().as_str().parse().unwrap();
                Event::BeginShift(id)
            }
        };

        records.push(Record {
            date: (date[0], date[1], date[2], date[3], date[4]),
            event: event,
        })
    }

    records.sort_by_key(|r| r.date);

    let mut active_id = 0;
    let mut start_sleep: Date = (0, 0, 0, 0, 0);
    let mut sleep_total = HashMap::<i32, i32>::new();
    let mut sleep_minute = HashMap::<(i32, i32), i32>::new();

    for record in records {
        match record.event {
            Event::BeginShift(id) => active_id = id,
            Event::FallAsleep => start_sleep = record.date,
            Event::WakeUp => {
                let (_, _, _, mut h, mut m) = start_sleep;
                let (_, _, _, h_end, m_end) = record.date;

                while h != h_end || m != m_end {
                    *sleep_total.entry(active_id).or_default() += 1;
                    *sleep_minute.entry((active_id, m)).or_default() += 1;

                    if m == 59 {
                        h += 1;
                        m = 0;
                    } else {
                        m += 1;
                    }
                }
            }
        }
    }

    let best_guard = *sleep_total.iter().max_by_key(|p| p.1).map(|p| p.0).unwrap();

    let minute = sleep_minute
        .iter()
        .filter(|p| (p.0).0 == best_guard)
        .max_by_key(|p| p.1)
        .map(|p| (p.0).1)
        .unwrap();

    println!(
        "best guard: {} ({} minutes asleep)",
        best_guard, sleep_total[&best_guard]
    );
    println!(
        "answer A: {} x {} = {}",
        minute,
        best_guard,
        minute * best_guard
    );

    let answer = sleep_minute
        .iter()
        .max_by_key(|p| p.1)
        .map(|p| p.0)
        .unwrap();

    println!(
        "answer B: {} x {} = {}",
        answer.0,
        answer.1,
        answer.0 * answer.1
    );
}
