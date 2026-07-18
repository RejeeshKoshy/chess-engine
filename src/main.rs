use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

// Masks to prevent pieces from wrapping around the board edges
const NOT_A_FILE: u64  = 0xFEFE_FEFE_FEFE_FEFE;
const NOT_AB_FILE: u64 = 0xFCFC_FCFC_FCFC_FCFC;
const NOT_H_FILE: u64  = 0x7F7F_7F7F_7F7F_7F7F;
const NOT_GH_FILE: u64 = 0x3F3F_3F3F_3F3F_3F3F;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub white_occupancy: u64,
    pub black_occupancy: u64,
    pub all_occupancy: u64,

    pub side_to_move: Color,
    pub en_passant_square: Option<u8>, 
    pub castling_rights: u8,           
    pub halfmove_clock: u8,            
    pub fullmove_number: u16,          
}

impl Board {
    pub fn new_starting_position() -> Self {
        let white_pawns   = 0x000000000000FF00;
        let white_knights = 0x0000000000000042;
        let white_bishops = 0x0000000000000024;
        let white_rooks   = 0x0000000000000081;
        let white_queens  = 0x0000000000000008;
        let white_king    = 0x0000000000000010;

        let black_pawns   = 0x00FF000000000000;
        let black_knights = 0x4200000000000000;
        let black_bishops = 0x2400000000000000;
        let black_rooks   = 0x8100000000000000;
        let black_queens  = 0x0800000000000000;
        let black_king    = 0x1000000000000000;

        let white_occupancy = white_pawns | white_knights | white_bishops | white_rooks | white_queens | white_king;
        let black_occupancy = black_pawns | black_knights | black_bishops | black_rooks | black_queens | black_king;

        Self {
            white_pawns, white_knights, white_bishops, white_rooks, white_queens, white_king,
            black_pawns, black_knights, black_bishops, black_rooks, black_queens, black_king,
            
            white_occupancy,
            black_occupancy,
            all_occupancy: white_occupancy | black_occupancy,
            
            side_to_move: Color::White,
            en_passant_square: None,
            castling_rights: 0b1111,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
    pub fn print(&self) {
        println!("\n  a b c d e f g h");
        
        // Loop from rank 7 down to 0 (Rank 8 down to Rank 1)
        for rank in (0..8).rev() {
            print!("{} ", rank + 1); // Print the rank number on the left
            
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = 1u64 << square; // Shift a 1 to the current square's position

                // Check which bitboard has a 1 at this position
                let c = if (self.white_pawns & bit) != 0 { 'P' }
                else if (self.white_knights & bit) != 0 { 'N' }
                else if (self.white_bishops & bit) != 0 { 'B' }
                else if (self.white_rooks & bit) != 0 { 'R' }
                else if (self.white_queens & bit) != 0 { 'Q' }
                else if (self.white_king & bit) != 0 { 'K' }
                else if (self.black_pawns & bit) != 0 { 'p' }
                else if (self.black_knights & bit) != 0 { 'n' }
                else if (self.black_bishops & bit) != 0 { 'b' }
                else if (self.black_rooks & bit) != 0 { 'r' }
                else if (self.black_queens & bit) != 0 { 'q' }
                else if (self.black_king & bit) != 0 { 'k' }
                else { '.' }; // Empty square

                print!("{} ", c);
            }
            println!("{}", rank + 1); // Print the rank number on the right
        }
        
        println!("  a b c d e f g h\n");
    }
}

pub fn generate_knight_attacks(square: u8) -> u64 {
    // Place a single Knight on an empty bitboard at the requested square
    let knight: u64 = 1u64 << square;
    let mut attacks: u64 = 0;

    // North-North-East (+17) and North-East-East (+10)
    // We use NOT_A_FILE and NOT_AB_FILE to ensure it didn't wrap around the right side
    attacks |= (knight << 17) & NOT_A_FILE;
    attacks |= (knight << 10) & NOT_AB_FILE;

    // South-East-East (-6) and South-South-East (-15)
    attacks |= (knight >> 6) & NOT_AB_FILE;
    attacks |= (knight >> 15) & NOT_A_FILE;

    // North-North-West (+15) and North-West-West (+6)
    attacks |= (knight << 15) & NOT_H_FILE;
    attacks |= (knight << 6) & NOT_GH_FILE;

    // South-West-West (-10) and South-South-West (-17)
    attacks |= (knight >> 10) & NOT_GH_FILE;
    attacks |= (knight >> 17) & NOT_H_FILE;

    attacks
}

pub fn generate_king_attacks(square: u8) -> u64 {
    let king: u64 = 1u64 << square;
    let mut attacks: u64 = 0;

    // East (+1) and West (-1)
    attacks |= (king << 1) & NOT_A_FILE;
    attacks |= (king >> 1) & NOT_H_FILE;

    // North (+8) and South (-8)
    // These don't need file masks because moving straight up/down can't wrap around the left/right edges
    attacks |= king << 8;
    attacks |= king >> 8;

    // North-East (+9) and North-West (+7)
    attacks |= (king << 9) & NOT_A_FILE;
    attacks |= (king << 7) & NOT_H_FILE;

    // South-East (-7) and South-West (-9)
    attacks |= (king >> 7) & NOT_A_FILE;
    attacks |= (king >> 9) & NOT_H_FILE;

    attacks
}

fn main() {
    let mut game_board = Board::new_starting_position();
    
    println!("=================================");
    println!("  Rust Chess Engine Initialized  ");
    println!("=================================");
    game_board.print();

    loop {
        // 1. Create a prompt showing whose turn it is
        let turn_label = match game_board.side_to_move {
            Color::White => "White",
            Color::Black => "Black",
        };
        
        print!("{} to move > ", turn_label);
        
        // Rust normally buffers output. This forces the prompt to show up immediately.
        io::stdout().flush().unwrap(); 

        // 2. Wait for the user to type something and press Enter
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Trim whitespace and the hidden newline character from the input
        let command = input.trim(); 

        // 3. Figure out what the user typed
        match command {
            "quit" | "exit" => {
                println!("Shutting down engine. Thanks for playing!");
                break;
            }
            "board" => game_board.print(),
            "test" => {
                // Let's test a King sitting on e4 (square 28)
                println!("King attacks from e4:");
                let attacks = generate_king_attacks(28);
                
                let mut dummy = Board::new_starting_position();
                
                // Wipe the board
                dummy.white_pawns = 0; dummy.white_knights = 0; dummy.white_bishops = 0; 
                dummy.white_rooks = 0; dummy.white_queens = 0; dummy.white_king = 0;
                dummy.black_pawns = 0; dummy.black_knights = 0; dummy.black_bishops = 0; 
                dummy.black_rooks = 0; dummy.black_queens = 0; dummy.black_king = 0;
                
                // Put the attacks on the white_king board so they print as 'K'
                dummy.white_king = attacks;
                dummy.print();
            }            "help" => {
                println!("Available commands:");
                println!("  board - Show the current board state");
                println!("  help  - Show this message");
                println!("  quit  - Exit the engine");
                println!("  (Move parsing like 'e4' or 'Nf3' is coming next!)");
            }
            "" => continue, // If they just hit Enter, do nothing and ask again
            _ => {
                // This is the fallback for any other text (which will eventually be our chess moves)
                println!("You tried to play '{}'. We need to teach the engine how to read that!", command);
            }
        }
    }
}
