pub fn crate_matrix(row_size: usize, col_size: usize) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = vec![vec![String::new(); col_size]; row_size];
    for row in 0..row_size {
        for col in 0..col_size {
            let num = row * col_size + col;
            matrix[row][col] = num.to_string();
        }
    }
    matrix
}
