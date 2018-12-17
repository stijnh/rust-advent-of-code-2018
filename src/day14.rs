use crate::common::read_file_lines;

pub fn run(_: &[&str]) {
    let line = read_file_lines("inputs/day14")[0].clone();
    let n = line.parse::<usize>().unwrap();
    let pattern = line
        .chars()
        .map(|x| x as i8 - '0' as i8)
        .collect::<Vec<_>>();

    let mut recipes = vec![3, 7];
    let mut index_a = 0;
    let mut index_b = 1;

    // just run suffient number iterations
    for _ in 0..50_000_000 {
        let sum = recipes[index_a] + recipes[index_b];

        if sum >= 10 {
            recipes.push(1);
            recipes.push(sum - 10);
        } else {
            recipes.push(sum);
        }

        index_a = (index_a + recipes[index_a] as usize + 1) % recipes.len();
        index_b = (index_b + recipes[index_b] as usize + 1) % recipes.len();
    }

    // find pattern at location n
    let answer = recipes[n..n + 10]
        .iter()
        .map(ToString::to_string)
        .collect::<String>();
    println!("answer A: {}", answer);

    // find position of given pattern
    let answer = recipes.windows(pattern.len()).position(|w| w == &*pattern);

    println!("answer B: {:?}", answer);
}
