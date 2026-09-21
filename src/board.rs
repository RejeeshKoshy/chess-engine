use crate::moves::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<char>,
    pub is_en_passant: bool,
}

impl Move {
    pub fn new(from: u8, to: u8) -> Self {
        Self { from, to, promotion: None, is_en_passant: false }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let promo = match self.promotion {
            Some(p) => format!("{}", p),
            None => String::new(),
        };
        write!(f, "{}{}{}", square_to_algebraic(self.from), square_to_algebraic(self.to), promo)
    }
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
        
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = 1u64 << square;

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
                else { '.' };

                print!("{} ", c);
            }
            println!("{}", rank + 1);
        }
        
        println!("  a b c d e f g h\n");
    }
    
    pub fn make_move(&mut self, m: Move) {
        let from_mask = 1u64 << m.from;
        let to_mask = 1u64 << m.to;
        let move_mask = from_mask | to_mask; 

        self.en_passant_square = None;

        if self.side_to_move == Color::White {
            if (self.white_pawns & from_mask) != 0 { 
                self.white_pawns ^= move_mask; 
                if let Some(promo) = m.promotion {
                    self.white_pawns &= !to_mask;
                    match promo {
                        'q' => self.white_queens |= to_mask,
                        'r' => self.white_rooks |= to_mask,
                        'b' => self.white_bishops |= to_mask,
                        'n' => self.white_knights |= to_mask,
                        _ => {}
                    }
                }
                else if m.is_en_passant {
                    self.black_pawns &= !(1u64 << (m.to - 8));
                }
                else if m.to == m.from + 16 {
                    self.en_passant_square = Some(m.from + 8);
                }
            }
            else if (self.white_knights & from_mask) != 0 { self.white_knights ^= move_mask; }
            else if (self.white_bishops & from_mask) != 0 { self.white_bishops ^= move_mask; }
            else if (self.white_rooks & from_mask) != 0 { self.white_rooks ^= move_mask; }
            else if (self.white_queens & from_mask) != 0 { self.white_queens ^= move_mask; }
            else if (self.white_king & from_mask) != 0 { 
                self.white_king ^= move_mask; 
                self.castling_rights &= !0b0011;
                
                if m.from == 4 && m.to == 6 {
                    self.white_rooks ^= (1u64 << 7) | (1u64 << 5);
                }
                if m.from == 4 && m.to == 2 {
                    self.white_rooks ^= (1u64 << 0) | (1u64 << 3);
                }
            }

            let clear_target = !to_mask;
            self.black_pawns &= clear_target;
            self.black_knights &= clear_target;
            self.black_bishops &= clear_target;
            self.black_rooks &= clear_target;
            self.black_queens &= clear_target;
            self.black_king &= clear_target;

            self.side_to_move = Color::Black;
        } else {
            if (self.black_pawns & from_mask) != 0 { 
                self.black_pawns ^= move_mask; 
                
                if let Some(promo) = m.promotion {
                    self.black_pawns &= !to_mask;
                    match promo {
                        'q' => self.black_queens |= to_mask,
                        'r' => self.black_rooks |= to_mask,
                        'b' => self.black_bishops |= to_mask,
                        'n' => self.black_knights |= to_mask,
                        _ => {}
                    }
                }
                else if m.is_en_passant {
                    self.white_pawns &= !(1u64 << (m.to + 8));
                }
                else if m.from == m.to + 16 {
                    self.en_passant_square = Some(m.from - 8);
                }
            }
            else if (self.black_knights & from_mask) != 0 { self.black_knights ^= move_mask; }
            else if (self.black_bishops & from_mask) != 0 { self.black_bishops ^= move_mask; }
            else if (self.black_rooks & from_mask) != 0 { self.black_rooks ^= move_mask; }
            else if (self.black_queens & from_mask) != 0 { self.black_queens ^= move_mask; }
            else if (self.black_king & from_mask) != 0 { 
                self.black_king ^= move_mask; 
                self.castling_rights &= !0b1100;
                
                if m.from == 60 && m.to == 62 {
                    self.black_rooks ^= (1u64 << 63) | (1u64 << 61);
                }
                if m.from == 60 && m.to == 58 {
                    self.black_rooks ^= (1u64 << 56) | (1u64 << 59);
                }
            } 

            let clear_target = !to_mask;
            self.white_pawns &= clear_target;
            self.white_knights &= clear_target;
            self.white_bishops &= clear_target;
            self.white_rooks &= clear_target;
            self.white_queens &= clear_target;
            self.white_king &= clear_target;

            self.side_to_move = Color::White;
        }

        self.white_occupancy = self.white_pawns | self.white_knights | self.white_bishops | self.white_rooks | self.white_queens | self.white_king;
        self.black_occupancy = self.black_pawns | self.black_knights | self.black_bishops | self.black_rooks | self.black_queens | self.black_king;
        self.all_occupancy = self.white_occupancy | self.black_occupancy;
    }
    
    pub fn generate_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(256);

        if self.side_to_move == Color::White {
            // WHITE KNIGHTS
            let mut knights = self.white_knights;
            while knights != 0 {
                let from_sq = knights.trailing_zeros() as u8;
                let mut attacks = generate_knight_attacks(from_sq) & !self.white_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1; 
                }
                knights &= knights - 1; 
            }

            // WHITE PAWNS
            let mut pawns = self.white_pawns;
            while pawns != 0 {
                let from_sq = pawns.trailing_zeros() as u8;
                let pushes = generate_white_pawn_pushes(from_sq, self.all_occupancy);
                let captures = generate_white_pawn_attacks(from_sq) & self.black_occupancy;
                let mut valid_pawn_moves = pushes | captures;
                
                while valid_pawn_moves != 0 {
                    let to_sq = valid_pawn_moves.trailing_zeros() as u8;
                    
                    if to_sq / 8 == 7 { 
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('q'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('r'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('b'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('n'), is_en_passant: false });
                    } else {
                        moves.push(Move::new(from_sq, to_sq));
                    }
                    
                    valid_pawn_moves &= valid_pawn_moves - 1;
                }

                if let Some(ep_sq) = self.en_passant_square {
                    let attacks = generate_white_pawn_attacks(from_sq);
                    if (attacks & (1u64 << ep_sq)) != 0 {
                        moves.push(Move { from: from_sq, to: ep_sq, promotion: None, is_en_passant: true });
                    }
                }
                pawns &= pawns - 1;
            }

            // WHITE BISHOPS
            let mut bishops = self.white_bishops;
            while bishops != 0 {
                let from_sq = bishops.trailing_zeros() as u8;
                let mut attacks = generate_bishop_attacks(from_sq, self.all_occupancy) & !self.white_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                bishops &= bishops - 1;
            }

            // WHITE ROOKS
            let mut rooks = self.white_rooks;
            while rooks != 0 {
                let from_sq = rooks.trailing_zeros() as u8;
                let mut attacks = generate_rook_attacks(from_sq, self.all_occupancy) & !self.white_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                rooks &= rooks - 1;
            }

            // WHITE QUEENS
            let mut queens = self.white_queens;
            while queens != 0 {
                let from_sq = queens.trailing_zeros() as u8;
                let mut attacks = generate_queen_attacks(from_sq, self.all_occupancy) & !self.white_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                queens &= queens - 1;
            }

            // WHITE KING
            let mut king = self.white_king;
            while king != 0 {
                let from_sq = king.trailing_zeros() as u8;
                let mut attacks = generate_king_attacks(from_sq) & !self.white_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                king &= king - 1; 
            }
            
            // WHITE CASTLING
            if (self.castling_rights & 1) != 0 {
                if (self.all_occupancy & ((1u64 << 5) | (1u64 << 6))) == 0 {
                    if !self.is_square_attacked(4, Color::Black) && 
                       !self.is_square_attacked(5, Color::Black) && 
                       !self.is_square_attacked(6, Color::Black) {
                        moves.push(Move::new(4, 6));
                    }
                }
            }
            if (self.castling_rights & 2) != 0 {
                if (self.all_occupancy & ((1u64 << 1) | (1u64 << 2) | (1u64 << 3))) == 0 {
                    if !self.is_square_attacked(4, Color::Black) && 
                       !self.is_square_attacked(3, Color::Black) && 
                       !self.is_square_attacked(2, Color::Black) {
                        moves.push(Move::new(4, 2));
                    }
                }
            }
            
       } else {
            // BLACK KNIGHTS
            let mut knights = self.black_knights;
            while knights != 0 {
                let from_sq = knights.trailing_zeros() as u8;
                let mut attacks = generate_knight_attacks(from_sq) & !self.black_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1; 
                }
                knights &= knights - 1; 
            }

            // BLACK PAWNS
            let mut pawns = self.black_pawns;
            while pawns != 0 {
                let from_sq = pawns.trailing_zeros() as u8;
                let pushes = generate_black_pawn_pushes(from_sq, self.all_occupancy);
                let captures = generate_black_pawn_attacks(from_sq) & self.white_occupancy;
                let mut valid_pawn_moves = pushes | captures;
                
                while valid_pawn_moves != 0 {
                    let to_sq = valid_pawn_moves.trailing_zeros() as u8;
                    
                    if to_sq / 8 == 0 { 
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('q'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('r'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('b'), is_en_passant: false });
                        moves.push(Move { from: from_sq, to: to_sq, promotion: Some('n'), is_en_passant: false });
                    } else {
                        moves.push(Move::new(from_sq, to_sq));
                    }
                    valid_pawn_moves &= valid_pawn_moves - 1;
                }

                if let Some(ep_sq) = self.en_passant_square {
                    let attacks = generate_black_pawn_attacks(from_sq);
                    if (attacks & (1u64 << ep_sq)) != 0 {
                        moves.push(Move { from: from_sq, to: ep_sq, promotion: None, is_en_passant: true });
                    }
                }
                pawns &= pawns - 1;
            }

            // BLACK BISHOPS
            let mut bishops = self.black_bishops;
            while bishops != 0 {
                let from_sq = bishops.trailing_zeros() as u8;
                let mut attacks = generate_bishop_attacks(from_sq, self.all_occupancy) & !self.black_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                bishops &= bishops - 1;
            }

            // BLACK ROOKS
            let mut rooks = self.black_rooks;
            while rooks != 0 {
                let from_sq = rooks.trailing_zeros() as u8;
                let mut attacks = generate_rook_attacks(from_sq, self.all_occupancy) & !self.black_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                rooks &= rooks - 1;
            }

            // BLACK QUEENS
            let mut queens = self.black_queens;
            while queens != 0 {
                let from_sq = queens.trailing_zeros() as u8;
                let mut attacks = generate_queen_attacks(from_sq, self.all_occupancy) & !self.black_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                queens &= queens - 1;
            }

            // BLACK KING
            let mut king = self.black_king;
            while king != 0 {
                let from_sq = king.trailing_zeros() as u8;
                let mut attacks = generate_king_attacks(from_sq) & !self.black_occupancy;
                while attacks != 0 {
                    let to_sq = attacks.trailing_zeros() as u8;
                    moves.push(Move::new(from_sq, to_sq));
                    attacks &= attacks - 1;
                }
                king &= king - 1;
            }
            
            // BLACK CASTLING
            if (self.castling_rights & 4) != 0 {
                if (self.all_occupancy & ((1u64 << 61) | (1u64 << 62))) == 0 {
                    if !self.is_square_attacked(60, Color::White) && 
                       !self.is_square_attacked(61, Color::White) && 
                       !self.is_square_attacked(62, Color::White) {
                        moves.push(Move::new(60, 62));
                    }
                }
            }
            if (self.castling_rights & 8) != 0 {
                if (self.all_occupancy & ((1u64 << 57) | (1u64 << 58) | (1u64 << 59))) == 0 {
                    if !self.is_square_attacked(60, Color::White) && 
                       !self.is_square_attacked(59, Color::White) && 
                       !self.is_square_attacked(58, Color::White) {
                        moves.push(Move::new(60, 58));
                    }
                }
            }
        }

        moves
    }
    
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let pseudo_moves = self.generate_pseudo_legal_moves();
        let mut legal_moves = Vec::with_capacity(pseudo_moves.len());

        for m in pseudo_moves {
            let mut temp_board = self.clone();
            temp_board.make_move(m);

            let (king_bitboard, enemy_color) = if self.side_to_move == Color::White {
                (temp_board.white_king, Color::Black)
            } else {
                (temp_board.black_king, Color::White)
            };

            if king_bitboard == 0 {
                continue; 
            }

            let king_square = king_bitboard.trailing_zeros() as u8;

            if !temp_board.is_square_attacked(king_square, enemy_color) {
                legal_moves.push(m);
            }
        }

        legal_moves
    }
    
   pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
        if by_color == Color::White {
            if (generate_black_pawn_attacks(square) & self.white_pawns) != 0 { return true; }
            if (generate_knight_attacks(square) & self.white_knights) != 0 { return true; }
            if (generate_king_attacks(square) & self.white_king) != 0 { return true; }
            if (generate_bishop_attacks(square, self.all_occupancy) & (self.white_bishops | self.white_queens)) != 0 { return true; }
            if (generate_rook_attacks(square, self.all_occupancy) & (self.white_rooks | self.white_queens)) != 0 { return true; }
        } else {
            if (generate_white_pawn_attacks(square) & self.black_pawns) != 0 { return true; }
            if (generate_knight_attacks(square) & self.black_knights) != 0 { return true; }
            if (generate_king_attacks(square) & self.black_king) != 0 { return true; }
            if (generate_bishop_attacks(square, self.all_occupancy) & (self.black_bishops | self.black_queens)) != 0 { return true; }
            if (generate_rook_attacks(square, self.all_occupancy) & (self.black_rooks | self.black_queens)) != 0 { return true; }
        }
        false
    }

    pub fn piece_at(&self, square: u8) -> Option<(char, Color)> {
    let bit = 1u64 << square;
    if (self.white_pawns & bit) != 0 { return Some(('P', Color::White)); }
    if (self.white_knights & bit) != 0 { return Some(('N', Color::White)); }
    if (self.white_bishops & bit) != 0 { return Some(('B', Color::White)); }
    if (self.white_rooks & bit) != 0 { return Some(('R', Color::White)); }
    if (self.white_queens & bit) != 0 { return Some(('Q', Color::White)); }
    if (self.white_king & bit) != 0 { return Some(('K', Color::White)); }
    
    // We return uppercase chars for Black too, so our SAN parser can easily compare them!
    if (self.black_pawns & bit) != 0 { return Some(('P', Color::Black)); } 
    if (self.black_knights & bit) != 0 { return Some(('N', Color::Black)); }
    if (self.black_bishops & bit) != 0 { return Some(('B', Color::Black)); }
    if (self.black_rooks & bit) != 0 { return Some(('R', Color::Black)); }
    if (self.black_queens & bit) != 0 { return Some(('Q', Color::Black)); }
    if (self.black_king & bit) != 0 { return Some(('K', Color::Black)); }
    None
}

    pub fn parse_san(&self, san: &str, legal_moves: &[Move]) -> Option<Move> {
        // 1. Remove fluff characters
        let clean_san = san.replace(['+', '#', 'x', '='], "");
        
        // 2. Handle Castling separately
        if clean_san == "O-O" || clean_san == "0-0" {
            return legal_moves.iter().find(|m| {
                self.piece_at(m.from).map(|(p, _)| p) == Some('K') && (m.to == 6 || m.to == 62)
            }).copied();
        }
        if clean_san == "O-O-O" || clean_san == "0-0-0" {
            return legal_moves.iter().find(|m| {
                self.piece_at(m.from).map(|(p, _)| p) == Some('K') && (m.to == 2 || m.to == 58)
            }).copied();
        }
    
        let mut chars: Vec<char> = clean_san.chars().collect();
        if chars.len() < 2 { return None; }
    
        // 3. Extract Promotion
        let mut promo = None;
        let last_char = *chars.last().unwrap();
        if "QRBNqrbn".contains(last_char) && chars.len() >= 3 {
            promo = Some(last_char.to_ascii_lowercase());
            chars.pop(); 
        }
    
        // 4. Extract Destination (the last two remaining chars)
        let rank_char = chars.pop().unwrap();
        let file_char = chars.pop().unwrap();
        let dest_str = format!("{}{}", file_char, rank_char);
        let to_sq = parse_square(&dest_str)?;
    
        // 5. Extract Piece Type (defaults to Pawn if no capital letter)
        let mut piece_type = 'P';
        if !chars.is_empty() && "KQRBN".contains(chars[0]) {
            piece_type = chars.remove(0);
        }
    
        // 6. Extract Disambiguation (e.g. the 'b' in Nbd7)
        let mut file_disambig = None;
        let mut rank_disambig = None;
        for c in chars {
            if ('a'..='h').contains(&c) { file_disambig = Some((c as u8) - b'a'); }
            if ('1'..='8').contains(&c) { rank_disambig = Some((c as u8) - b'1'); }
        }
    
        // 7. Filter the legal moves!
        let mut matching_moves = vec![];
        for &m in legal_moves {
            if m.to != to_sq { continue; }
            if m.promotion != promo { continue; }
            
            if let Some((p_type, color)) = self.piece_at(m.from) {
                if color != self.side_to_move { continue; }
                if p_type != piece_type { continue; }
                
                let from_file = m.from % 8;
                let from_rank = m.from / 8;
                
                // If the user provided disambiguation, make sure it matches
                if let Some(f) = file_disambig { if from_file != f { continue; } }
                if let Some(r) = rank_disambig { if from_rank != r { continue; } }
                
                matching_moves.push(m);
            }
        }
    
        // If exactly one move matches, we found our move!
        if matching_moves.len() == 1 { Some(matching_moves[0]) } else { None }
    }
    
}

pub fn parse_square(s: &str) -> Option<u8> {
    let mut chars = s.chars();
    let file_char = chars.next()?;
    let rank_char = chars.next()?;

    if !('a'..='h').contains(&file_char) || !('1'..='8').contains(&rank_char) {
        return None;
    }

    let file = (file_char as u8) - b'a';
    let rank = (rank_char as u8) - b'1';
    
    Some(rank * 8 + file) 
}

pub fn square_to_algebraic(square: u8) -> String {
    let file = square % 8;
    let rank = square / 8;
    
    let file_char = (file + b'a') as char;
    let rank_char = (rank + b'1') as char;
    
    format!("{}{}", file_char, rank_char)
}