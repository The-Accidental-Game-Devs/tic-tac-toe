use std::io;

fn main() {
    let player1_symbol: String = String::from("X");
    let player2_symbol: String = String::from("O");
    let players_symbol: [String; 2] = [player1_symbol, player2_symbol];
    let mut player_turn: usize = 0;

    let row_size: usize = 3;
    let col_size: usize = 3;
    let game_length: usize = row_size * col_size;
    let connect_amount: usize = 3;
    let mut turn_count: usize = 0;
    let mut game_board = crate_matrix(row_size, col_size);

    'game_loop: loop {
        // Print game
        print_game(&game_board);

        // Print prompt
        println!("Player-{} turn.", players_symbol[player_turn]);
        println!("Enter empty index between 0-{}: ", game_length - 1);

        // Read input
        let mut selected_index = String::new();

        io::stdin()
            .read_line(&mut selected_index)
            .expect("Failed to read line");

        // Convert selected_index to integer
        let selected_index: usize = match selected_index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter positive integer number.");
                continue;
            }
        };

        // Check if input is within valid range (0 to game_lenght - 1).
        if selected_index > game_length - 1 {
            println!("Please enter a number between 0 and {}.", game_length - 1);
            continue;
        }

        // Convert the input index to row and column indices in the row_size x col_size matrix
        let selected_row = selected_index / row_size;
        let selected_col = selected_index % col_size;

        // Check if the position is already taken
        if players_symbol.contains(&game_board[selected_row][selected_col]) {
            println!("This spot is already taken. Please choose another index");
            continue;
        }

        // Mark the spot with the current player's symbol
        game_board[selected_row][selected_col] = players_symbol[player_turn].clone();
        turn_count += 1;

        // Check win
        if check_win(selected_row, selected_col, connect_amount, &game_board) {
            print_game(&game_board);
            println!("Player-{} win!.", players_symbol[player_turn]);
            break 'game_loop;
        }

        // If no win and turn_count is equal to game_length, it will be draw.
        if turn_count >= game_length {
            print_game(&game_board);
            println!("Draw");
            break 'game_loop;
        }

        player_turn += 1;
        player_turn %= players_symbol.len();
    }
}

fn crate_matrix(row_size: usize, col_size: usize) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = vec![vec![String::new(); col_size]; row_size];
    for row in 0..row_size {
        for col in 0..col_size {
            let num = row * col_size + col;
            matrix[row][col] = num.to_string();
        }
    }
    matrix
}

fn print_game(matrix: &Vec<Vec<String>>) {
    println!("");

    for (i, row) in matrix.iter().enumerate() {
        println!("{}", row.join(" | "));

        if i < matrix.len() - 1 {
            println!("{}", "-".repeat(row.join(" | ").len()))
        }
    }
    println!("");
}

fn is_index_out_of_bounds(row: i32, col: i32, row_size: usize, col_size: usize) -> bool {
    if row < 0 || col < 0 {
        return true;
    }

    if row >= row_size as i32 || col >= col_size as i32 {
        return true;
    }

    false
}

fn get_connected_paths(
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

fn check_win(
    selected_row: usize,
    selected_col: usize,
    connect_amount: usize,
    matrix: &Vec<Vec<String>>,
) -> bool {
    let connected_index = get_connected_paths(selected_row, selected_col, &matrix);
    for row in connected_index {
        if row.len() >= connect_amount {
            return true;
        }
    }

    false
}
