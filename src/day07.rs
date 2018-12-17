use crate::common::read_file_lines;
use regex::Regex;
use std::collections::HashSet;

fn part_a(edges: &[(char, char)]) {
    let mut pending = HashSet::new();
    for (a, b) in edges.iter().cloned() {
        pending.insert(a);
        pending.insert(b);
    }

    let mut order = "".to_string();

    while pending.len() > 0 {
        let mut candidates = pending.clone();

        for (a, b) in edges.iter() {
            if pending.contains(a) {
                candidates.remove(b);
            }
        }

        let candidate = candidates.iter().min().unwrap();

        order.push(*candidate);
        pending.remove(candidate);
    }

    println!("answer A: {}", order);
}

fn part_b(edges: &[(char, char)]) {
    const NUM_WORKERS: usize = 5;
    let mut workers: [(i32, Option<char>); NUM_WORKERS] = [(0, None); NUM_WORKERS];
    let mut active_jobs = HashSet::new();
    let mut pending_jobs = HashSet::new();

    for (a, b) in edges.iter().cloned() {
        pending_jobs.insert(a);
        pending_jobs.insert(b);
    }

    let mut time = 0;

    while active_jobs.len() + pending_jobs.len() > 0 {
        // Find earliest finishing worker and pop his job
        let worker_opt = workers
            .iter_mut()
            .filter(|w| w.1.is_some())
            .min_by_key(|w| w.0);

        if let Some(w) = worker_opt {
            time = w.0;
            let job = w.1.take().unwrap();

            active_jobs.remove(&job);
        }

        // Assign candidate jobs to idle workers
        let mut candidate_jobs = pending_jobs.clone();

        for (a, b) in edges.iter() {
            if pending_jobs.contains(a) || active_jobs.contains(a) {
                candidate_jobs.remove(b);
            }
        }

        let idle_workers = workers.iter_mut().filter(|w| w.1.is_none());

        for (worker, job) in idle_workers.zip(candidate_jobs) {
            pending_jobs.remove(&job);
            active_jobs.insert(job);

            let job_time = 61 + (job as i32 - 'A' as i32);
            *worker = (time + job_time, Some(job));
        }
    }

    let total_time = workers.iter().map(|w| w.0).max().unwrap();

    println!("answer B: {:?}", total_time);
}

pub fn run(_: &[&str]) {
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    let mut edges = vec![];

    for line in read_file_lines("inputs/day7") {
        let matches = re.captures(&line).unwrap();
        let src = matches.get(1).unwrap().as_str().chars().next().unwrap();
        let dst = matches.get(2).unwrap().as_str().chars().next().unwrap();

        edges.push((src, dst));
    }

    part_a(&edges);
    part_b(&edges);
}
