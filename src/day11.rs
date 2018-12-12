// Compute value of cell at coordinates (x, y)
fn compute_cell(x: i64, y: i64) -> i64 {
    const SERIAL_ID: usize = 3031;

    let rack_id = x + 10;
    let level = ((rack_id * y) + SERIAL_ID as i64) * rack_id;
    let power = (level / 100) % 10 - 5;

    power
}

// Compute cumulative summed grid where entry (x, y) holds the sum over the cells (0..x, 0..y)
fn compute_cumsum_grid(size: usize) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; size + 1]; size + 1];

    for i in 0..size {
        for j in 0..size {
            let cell = compute_cell(i as i64 + 1, j as i64 + 1);

            grid[i + 1][j + 1] = cell + grid[i][j + 1] + grid[i + 1][j] - grid[i][j];
        }
    }

    grid
}

// Compute the sum over the cells (x..x+size, y..y+size)
fn compute_area(grid: &[Vec<i64>], lower: (usize, usize), size: usize) -> i64 {
    let upper = (lower.0 + size, lower.1 + size);
    let mut result = 0;
    result += grid[upper.0][upper.1];
    result -= grid[lower.0][upper.1];
    result -= grid[upper.0][lower.1];
    result += grid[lower.0][lower.1];
    result
}

pub fn run(_: &[&str]) {
    const GRID_SIZE: usize = 300;

    let sum_grid = compute_cumsum_grid(GRID_SIZE);

    let mut best = (0, 0, 0);

    for i in 0..(GRID_SIZE - 2) {
        for j in 0..(GRID_SIZE - 2) {
            let score = compute_area(&sum_grid, (i, j), 3);

            if score > best.0 {
                best = (score, i + 1, j + 1);
            }
        }
    }

    println!("answer A: {:?}", best);

    let mut best = (0, 0, 0, 0);
    for size in 1..GRID_SIZE {
        for i in 0..(GRID_SIZE - size + 1) {
            for j in 0..(GRID_SIZE - size + 1) {
                let score = compute_area(&sum_grid, (i, j), size);

                if score > best.0 {
                    best = (score, i + 1, j + 1, size);
                }
            }
        }
    }

    println!("answer B: {:?}", best);
}
