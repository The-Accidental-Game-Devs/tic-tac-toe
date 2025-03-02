use std::io;
mod matrix;
mod path_finder;

fn main() {
    let player1_symbol: String = String::from("X");
    let player2_symbol: String = String::from("O");
    let player_symbols: [String; 2] = [player1_symbol, player2_symbol];
    let mut player_turn: usize = 0;

    let min_row_col_size: usize = 3;
    let max_row_col_size: usize = 5;
    let max_cell_width: usize = 3;

    'main: loop {
        // Initialize or restart turn_count
        let mut turn_count: usize = 0;

        // Ask player to enter row_col_size
        let row_col_size = num_input(
            &format!(
                "Enter the size for both rows and columns between ({}-{}):",
                min_row_col_size, max_row_col_size
            ),
            min_row_col_size,
            max_row_col_size,
        );

        // Caculated board size base on row_col_size^2
        let board_size: usize = row_col_size * row_col_size;

        // Ask player to enter connect_amount
        let connect_amount = num_input(
            &format!(
                "Enter connect amount between ({}-{}):",
                min_row_col_size, row_col_size
            ),
            min_row_col_size,
            row_col_size,
        );

        // Crate game board base on board size
        let mut game_board = matrix::crate_matrix(row_col_size, row_col_size);

        'game_loop: loop {
            print_game(&game_board, max_cell_width);

            // Ask plyer to enter index to place with current player's symbol
            println!("Player-{} turn.", player_symbols[player_turn]);

            let selected_index = num_input(
                &format!("Enter empty index bewtween ({}-{}):", 0, board_size - 1),
                0,
                board_size - 1,
            );

            // Convert the selected index to row and column indices in the row_size * col_size matrix
            // This only work for even matris.
            let selected_row = selected_index / row_col_size;
            let selected_col = selected_index % row_col_size;

            // Check if the position is already taken
            if player_symbols.contains(&game_board[selected_row][selected_col]) {
                println!("This spot is already taken. Please choose other index.");
                continue;
            }

            // Mark the spot with current player's symbol
            game_board[selected_row][selected_col] = player_symbols[player_turn].clone();

            // Count turn
            turn_count += 1;

            let mut end_game = false;

            // Check win
            if is_w(selected_row, selected_col, connect_amount, &game_board) {
                print_game(&game_board, max_cell_width);
                println!("Player-{} win.", player_symbols[player_turn]);
                end_game = true;
            }

            // If no win and turn_count is equal to boaed size  it will be a draw
            if turn_count >= board_size {
                print_game(&game_board, max_cell_width);
                println!("Draw.");
                end_game = true;
            }

            // Cycle through players
            player_turn += 1;
            player_turn %= player_symbols.len();

            if end_game {
                break 'game_loop;
            }
        }

        loop {
            // Ask player to restart or end the game
            println!("Enter q to quit or r to restart the game:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim() {
                "r" => break,
                "q" => break 'main,
                _ => println!("Invalid input, please enter 'q' or 'r'"),
            };
        }
    }
}

fn num_input(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        println!("{prompt}");
        let mut input_num = String::new();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line");

        let input_num: usize = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter integer number.");
                continue;
            }
        };

        if input_num < min || input_num > max {
            println!("Please enter a number between {} and {}.", min, max);
            continue;
        }

        return input_num;
    }
}

fn print_game(game_board: &Vec<Vec<String>>, max_cell_width: usize) {
    for (i, row) in game_board.iter().enumerate() {
        // Format each cell to match `max_cell_width`
        let mut formatted_row: Vec<String> = vec![];
        // Format value by centering it with a width of `max_cell_width`
        for value in row {
            let formatted_value = format!("{:^max_cell_width$}", value);
            formatted_row.push(formatted_value);
        }

        // Print the row with spaces around "|"
        println!("{}", formatted_row.join(" | "));

        // Print separator if it's not the last row
        if i < game_board.len() - 1 {
            println!("{}", "-".repeat(formatted_row.join(" | ").len()))
        }
    }
}

fn is_w(
    selected_row: usize,
    selected_col: usize,
    connect_amount: usize,
    matrix: &Vec<Vec<String>>,
) -> bool {
    let connected_paths = path_finder::get_connected_paths(selected_row, selected_col, matrix);
    // Check if one of the connected_paths length is greater than or equal to connect_amount
    for row in connected_paths {
        if row.len() >= connect_amount {
            return true;
        }
    }
    false
}
