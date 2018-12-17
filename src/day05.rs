use crate::common::read_file_lines;

fn invert_case(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

pub fn shrink_polymere(mut vec: Vec<char>) -> Vec<char> {
    loop {
        let mut read_index = 0;
        let mut write_index = 0;

        while read_index < vec.len() {
            let a = vec.get(read_index).unwrap_or(&'\0');
            let b = vec.get(read_index + 1).unwrap_or(&'\0');

            if invert_case(*a) == *b {
                read_index += 2;
            } else {
                vec[write_index] = vec[read_index];
                read_index += 1;
                write_index += 1;
            }
        }

        if read_index != write_index {
            vec.resize(write_index, '\0');
        } else {
            break vec;
        }
    }
}

pub fn run(_: &[&str]) {
    let mut vec = read_file_lines("inputs/day5")[0]
        .chars()
        .collect::<Vec<_>>();
    vec = shrink_polymere(vec);

    println!("answer A: {:?}", vec.len());

    let mut best = ('\0', vec.len());
    let mut options = vec.clone();
    options.sort();
    options.dedup();

    for c in options {
        let mut filtered_vec = vec
            .iter()
            .cloned()
            .filter(|x| *x != c && *x != invert_case(c))
            .collect::<Vec<_>>();

        filtered_vec = shrink_polymere(filtered_vec);

        println!(" {}: {}", c, filtered_vec.len());

        if filtered_vec.len() < best.1 {
            best = (c, filtered_vec.len());
        }
    }

    println!("answer B: {:?}", best);
}
