#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

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

fn main() {
    let game_board = Board::new_starting_position();
    
    println!("Initial Position:");
    game_board.print();
}
