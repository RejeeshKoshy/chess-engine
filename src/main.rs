#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    // Creates a completely empty board (all 0s)
    pub fn empty() -> Self {
        Bitboard(0)
    }

    // Flips a specific bit to 1 (places a piece)
    pub fn set_square(&mut self, square: u8) {
        // square is an index from 0 to 63
        self.0 |= 1u64 << square;
    }

    // Checks if a bit is 1 (is a piece there?)
    pub fn has_piece(&self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    // Clears a specific square to 0 (removes a piece)
    pub fn clear_square(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }
}
