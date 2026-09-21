// Masks to prevent pieces from wrapping around the board edges
const NOT_A_FILE: u64  = 0xFEFE_FEFE_FEFE_FEFE;
const NOT_AB_FILE: u64 = 0xFCFC_FCFC_FCFC_FCFC;
const NOT_H_FILE: u64  = 0x7F7F_7F7F_7F7F_7F7F;
const NOT_GH_FILE: u64 = 0x3F3F_3F3F_3F3F_3F3F;

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

pub fn generate_white_pawn_attacks(square: u8) -> u64 {
    let pawn: u64 = 1u64 << square;
    let mut attacks: u64 = 0;

    // North-West (+7) and North-East (+9)
    // We use file masks so an H-file pawn doesn't wrap around and attack the A-file!
    attacks |= (pawn << 7) & NOT_H_FILE;
    attacks |= (pawn << 9) & NOT_A_FILE;

    attacks
}

pub fn generate_white_pawn_pushes(square: u8, occupancy: u64) -> u64 {
    let pawn: u64 = 1u64 << square;
    let mut pushes: u64 = 0;

    // 1. Single Push (+8)
    // The pawn can only move if the square 8 bits ahead is NOT occupied
    let single_push = (pawn << 8) & !occupancy;
    pushes |= single_push;

    // 2. Double Push (+16)
    // Only valid if:
    // A) The single push square was empty (single_push != 0)
    // B) The pawn is on Rank 2. We check this by seeing if square / 8 == 1 (squares 8 through 15)
    if single_push != 0 && (square / 8 == 1) {
        let double_push = (pawn << 16) & !occupancy;
        pushes |= double_push;
    }

    pushes
}

pub fn generate_black_pawn_attacks(square: u8) -> u64 {
    let pawn: u64 = 1u64 << square;
    let mut attacks: u64 = 0;

    // South-East (-7) and South-West (-9)
    attacks |= (pawn >> 7) & NOT_A_FILE;
    attacks |= (pawn >> 9) & NOT_H_FILE;

    attacks
}

pub fn generate_black_pawn_pushes(square: u8, occupancy: u64) -> u64 {
    let pawn: u64 = 1u64 << square;
    let mut pushes: u64 = 0;

    // Single Push (-8)
    let single_push = (pawn >> 8) & !occupancy;
    pushes |= single_push;

    // Double Push (-16)
    // Pawn must be on Rank 7 (squares 48 through 55)
    if single_push != 0 && (square / 8 == 6) {
        let double_push = (pawn >> 16) & !occupancy;
        pushes |= double_push;
    }

    pushes
}

pub fn generate_rook_attacks(square: u8, occupancy: u64) -> u64 {
    let mut attacks: u64 = 0;
    
    // North (+8)
    let mut ray = (1u64 << square) << 8;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; } // Hit a piece, stop!
        ray <<= 8;
    }
    
    // South (-8)
    let mut ray = (1u64 << square) >> 8;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray >>= 8;
    }

    // East (+1) - Requires edge masking to not wrap around the board
    let mut ray = ((1u64 << square) << 1) & NOT_A_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray << 1) & NOT_A_FILE;
    }

    // West (-1) - Requires edge masking
    let mut ray = ((1u64 << square) >> 1) & NOT_H_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray >> 1) & NOT_H_FILE;
    }

    attacks
}

pub fn generate_bishop_attacks(square: u8, occupancy: u64) -> u64 {
    let mut attacks: u64 = 0;

    // North-East (+9)
    let mut ray = ((1u64 << square) << 9) & NOT_A_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray << 9) & NOT_A_FILE;
    }

    // North-West (+7)
    let mut ray = ((1u64 << square) << 7) & NOT_H_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray << 7) & NOT_H_FILE;
    }

    // South-East (-7)
    let mut ray = ((1u64 << square) >> 7) & NOT_A_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray >> 7) & NOT_A_FILE;
    }

    // South-West (-9)
    let mut ray = ((1u64 << square) >> 9) & NOT_H_FILE;
    while ray != 0 {
        attacks |= ray;
        if (ray & occupancy) != 0 { break; }
        ray = (ray >> 9) & NOT_H_FILE;
    }

    attacks
}

pub fn generate_queen_attacks(square: u8, occupancy: u64) -> u64 {
    generate_rook_attacks(square, occupancy) | generate_bishop_attacks(square, occupancy)
}