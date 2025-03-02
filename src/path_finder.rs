fn is_index_out_of_bounds(row: i32, col: i32, row_size: usize, col_size: usize) -> bool {
    if row < 0 || col < 0 {
        return true;
    }

    if row >= row_size as i32 || col >= col_size as i32 {
        return true;
    }

    false
}

pub fn get_connected_paths(
    start_row: usize,
    start_col: usize,
    matrix: &Vec<Vec<String>>,
) -> Vec<Vec<Vec<usize>>> {
    let directions = [
        [
            [-1, 0], // Up
            [1, 0],  // Down
        ], // Vertical movement
        [
            [0, -1], // Left
            [0, 1],  // Right
        ], // Horizontal movement
        [
            [-1, 1], // Top-right
            [1, -1], // Bottom-left
        ], // Main diagonal movement
        [
            [-1, -1], // Top-left
            [1, 1],   // Bottom-right
        ], // Anti-diagonal movement
    ];

    // This stores all connected paths
    let mut connected_paths: Vec<Vec<Vec<usize>>> = vec![vec![]; 4];

    for (dir_index, direction_group) in directions.iter().enumerate() {
        // Start each path with the initial position
        connected_paths[dir_index].push(vec![start_row, start_col]);

        for direction in direction_group {
            let mut curr_row = start_row; // Current row index
            let mut curr_col = start_col; // Current column index
            let mut next_row = start_row as i32 + direction[0]; // Next row to check
            let mut next_col = start_col as i32 + direction[1]; // Next column to check

            loop {
                // Stop if the next position is out of bounds
                if is_index_out_of_bounds(next_row, next_col, matrix.len(), matrix[start_row].len())
                {
                    break;
                }

                /* If the next cell contains the same value as the current cell,
                continue in this direction and add the new position to the path. */
                if matrix[curr_row][curr_col] == matrix[next_row as usize][next_col as usize] {
                    curr_row = next_row as usize;
                    curr_col = next_col as usize;
                    next_row += direction[0];
                    next_col += direction[1];
                    connected_paths[dir_index].push(vec![curr_row, curr_col]);
                } else {
                    break;
                }
            }
        }
    }
    connected_paths
}
