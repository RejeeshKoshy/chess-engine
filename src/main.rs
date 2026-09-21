mod moves;
mod board;

use std::io::{self, Write};
use board::{Board, Color};

fn main() {
    let mut game_board = Board::new_starting_position();
    
    println!("=================================");
    println!("  Rust Chess Engine Initialized  ");
    println!("=================================");
    // game_board.print();

    loop {
        // 1. Generate all moves for the current turn exactly once
        let legal_moves = game_board.generate_legal_moves();
        
        // 2. Game Over Detection
        if legal_moves.is_empty() {
            // Find out where the current King is sitting
            let (king_board, enemy_color) = match game_board.side_to_move {
                Color::White => (game_board.white_king, Color::Black),
                Color::Black => (game_board.black_king, Color::White),
            };
            
            let king_sq = king_board.trailing_zeros() as u8;
            
            // If the King is attacked, it's Checkmate. Otherwise, Stalemate!
            if game_board.is_square_attacked(king_sq, enemy_color) {
                println!("CHECKMATE! {:?} wins the game!", enemy_color);
            } else {
                println!("STALEMATE! The game is a draw.");
            }
            break; // Exit the loop, ending the program
        }

        // 3. Prompt for input
        let turn_label = match game_board.side_to_move {
            Color::White => "White",
            Color::Black => "Black",
        };
        
        print!("{} to move > ", turn_label);
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim(); 

        match command {
            "moves" => {
                // Reuse the list we generated at the top!
                println!("Found {} legal moves:", legal_moves.len());
                for m in &legal_moves {
                    println!("{}", m);
                }
            }
            "quit" | "exit" => {
                println!("Shutting down engine. Thanks for playing!");
                break;
            }
            "board" => game_board.print(),
            "help" => {
                println!("Available commands:");
                println!("  board - Show the current board state");
                println!("  moves - Show all legal moves for the current turn");
                println!("  help  - Show this message");
                println!("  quit  - Exit the engine");
                println!("  (Or just type a move like 'e2e4' to play!)");
            }
            "" => continue,
            _ => {
                // Pass the input string and our pre-calculated legal moves to our new parser
                if let Some(matched_move) = game_board.parse_san(command, &legal_moves) {
                    game_board.make_move(matched_move);
                    // game_board.print();
                } else {
                    println!("Invalid move. Try algebraic notation (e4, Nf3, O-O, exd5).");
                }
            }
        }
    }
}