pub fn run(_: &[&str]) {
    const SERIAL_ID: usize = 3031;
    const GRID_SIZE: usize = 300;

    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let (x, y) = (i as i64 + 1, j as i64 + 1);

            let rack_id = x + 10;
            let level = ((rack_id * y) + SERIAL_ID as i64) * rack_id;
            let power = (level / 100) % 10 - 5;

            grid[i][j] = power;
        }
    }

    let mut sum_grid = vec![vec![0; GRID_SIZE + 1]; GRID_SIZE + 1];
    for i in 1..(GRID_SIZE+1) {
        for j in 1..(GRID_SIZE+1) {
            sum_grid[i][j] =
                grid[i - 1][j - 1]
                + sum_grid[i - 1][j] 
                + sum_grid[i][j - 1] 
                - sum_grid[i -1][j - 1];
        }
    }

    let mut best = (0, 0, 0);

    for i in 0..(GRID_SIZE - 2) {
        for j in 0..(GRID_SIZE - 2) {
            let mut score = 
                sum_grid[i + 3][j + 3] 
                - sum_grid[i][j + 3]
                - sum_grid[i + 3][j]
                + sum_grid[i][j];

            if score > best.0 {
                best = (score, i + 1, j + 1);
            }
        }
    }

    println!("answer A: {:?}", best);

    let mut best = (0, 0, 0, 0);
    for size in 1..GRID_SIZE {
        for i in 0..(GRID_SIZE - size) {
            for j in 0..(GRID_SIZE - size) {
                let mut score = 
                    sum_grid[i + size][j + size] 
                    - sum_grid[i][j + size]
                    - sum_grid[i + size][j]
                    + sum_grid[i][j];

                if score > best.0 {
                    best = (score, i + 1, j + 1, size);
                }
            }
        }
    }

    println!("answer B: {:?}", best);
}
