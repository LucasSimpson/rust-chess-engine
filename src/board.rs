pub type BoardSlot = u8;

const EMPTY: BoardSlot = 0;
const PAWN: BoardSlot = 1;
const ROOK: BoardSlot = 2;
const KNIGHT: BoardSlot = 4;
const BISHOP: BoardSlot = 8;
const KING: BoardSlot = 16;
const QUEEN: BoardSlot = 32;
const UNIT_MASK: u8 = 127;
const TEAM_MASK: u8 = 128;
const TEAM_WHITE: u8 = 128;
const TEAM_BLACK: u8 = 0;

fn char_for_slot(slot: BoardSlot) -> char {
    match slot & UNIT_MASK {
        EMPTY => ' ',
        PAWN => if slot & TEAM_MASK > 0 { 'P' } else { 'p' },
        ROOK => if slot & TEAM_MASK > 0 { 'R' } else { 'r' },
        KNIGHT => if slot & TEAM_MASK > 0 { 'N' } else { 'n' },
        BISHOP => if slot & TEAM_MASK > 0 { 'B' } else { 'b' },
        KING => if slot & TEAM_MASK > 0 { 'K' } else { 'k' },
        QUEEN => if slot & TEAM_MASK > 0 { 'Q' } else { 'q' },
        _ => 'X',
    }
}

fn slot_from_char(c: char) -> BoardSlot {
    match c {
        'p' => TEAM_BLACK | PAWN,
        'r' => TEAM_BLACK | ROOK,
        'n' => TEAM_BLACK | KNIGHT,
        'b' => TEAM_BLACK | BISHOP,
        'k' => TEAM_BLACK | KING,
        'q' => TEAM_BLACK | QUEEN,
        'P' => TEAM_WHITE | PAWN,
        'R' => TEAM_WHITE | ROOK,
        'N' => TEAM_WHITE | KNIGHT,
        'B' => TEAM_WHITE | BISHOP,
        'K' => TEAM_WHITE | KING,
        'Q' => TEAM_WHITE | QUEEN,
        _ => EMPTY,
    }
}

#[derive(Debug, Clone)]
pub struct ChessMove {
    from_ind: (i32, i32),
    to_ind: (i32, i32),
    promotion: u8,
}

impl ChessMove {
    pub fn from_long_algebraic_notation(notation: &str) -> ChessMove {
        let parts: Vec<char> = notation.chars().collect();
        let mut cmove = ChessMove{
            from_ind: (ChessMove::letter_to_column(parts[0]), ChessMove::parse_rank(parts[1])),
            to_ind: (ChessMove::letter_to_column(parts[2]), ChessMove::parse_rank(parts[3])),
            promotion: EMPTY
        };

        if parts.len() == 5 {
            cmove.promotion = slot_from_char(parts[4]);
        }

        cmove
    }

    pub fn from_delta(x: i32, y: i32, dx: i32, dy: i32) -> Option<ChessMove> {
        if ChessMove::within(x + dx) && ChessMove::within(y + dy) {
            Some(ChessMove{
                from_ind: (x, y),
                to_ind: (x + dx, y + dy),
                promotion: 0,
            })
        } else {
            None
        }
    }

    pub fn to_long_algebraic_notation(&self) -> String {
        format!(
            "{}{}{}{}",
            ChessMove::column_to_letter(self.from_ind.0),
            self.from_ind.1 + 1,
            ChessMove::column_to_letter(self.to_ind.0),
            self.to_ind.1 + 1,
        ).to_string()
    }

    fn parse_rank(rank: char) -> i32 {
        (rank.to_digit(10).unwrap_or(255) - 1) as i32
    }

    fn column_to_letter(col: i32) -> char {
        match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => 'X',
        }
    }

    fn letter_to_column(letter: char) -> i32 {
        match letter {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => 255,
        }
    }

    fn within(x: i32) -> bool {
        x >= 0 && x <= 7
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    board: [[BoardSlot; 8]; 8],
    is_whites_move: bool,
    whos_turn: u8,
    half_move_clock: i32,
    full_move_clock: i32,
    white_ks_castle: bool,
    white_qs_castle: bool,
    black_ks_castle: bool,
    black_qs_castle: bool,
    white_in_check: bool,
    black_in_check: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            whos_turn: TEAM_BLACK,
            is_whites_move: false,
            half_move_clock: 0,
            full_move_clock: 0,
            white_ks_castle: false,
            white_qs_castle: false,
            black_ks_castle: false,
            black_qs_castle: false,
            white_in_check: false,
            black_in_check: false,
        }
    }

    pub fn is_whites_move(&self) -> bool {
        return self.is_whites_move;
    }

    pub fn reset_from_fen(mut self, fen: [&str; 6]) -> std::result::Result<Self, String> {
        // parse the board state
        let ranks: Vec<&str> = fen[0].split("/").collect();

        if ranks.len() != 8 {
            return Err(String::from("Illegal length of ranks"))
        }

        // fen is given in reverse rank orders
        for i in 0..8 {
            let mut slots: Vec<BoardSlot> = Vec::new();
            for elem in ranks[i].chars() {
                match elem.to_digit(10) {
                    Some(n) => slots.extend((0..n).map(|_| EMPTY)),
                    None => slots.push(slot_from_char(elem)),
                }
            }

            if slots.len() != 8 {
                return Err(String::from("illegal length of row"))
            }

            for j in 0..8 {
                // we mirror the i index here because they arrive in reverse-rank order
                self.board[j][7 - i] = slots[j]
            }
        }

        // parse who's turn it is
        self.is_whites_move = fen[1] == "w";
        self.whos_turn = if self.is_whites_move { TEAM_WHITE } else { TEAM_BLACK };

        // parse castling
        self.white_ks_castle = false;
        self.white_qs_castle = false;
        self.black_ks_castle = false;
        self.black_qs_castle = false;
        for x in fen[2].chars() {
            match x {
                'K' => self.white_ks_castle = true,
                'Q' => self.white_qs_castle = true,
                'k' => self.black_ks_castle = true,
                'q' => self.black_qs_castle = true,
                _ => ()
            }
        }

        // parse en-passant
        // TODO lol why

        // parse half-move clock
        match fen[4].parse::<i32>().ok() {
            Some(n) => self.half_move_clock = n,
            None => return Err(String::from("Couldn't parse half-move clock"))
        }

        // parse full-move clock
        match fen[5].parse::<i32>().ok() {
            Some(n) => self.full_move_clock = n,
            None => return Err(String::from("Couldn't parse half-move clock"))
        }

        Ok(self)
    }

    pub fn apply_move_into(self, chess_move: ChessMove) -> Board {
        self.apply_move(&chess_move)
    }

    pub fn apply_move(mut self, chess_move: &ChessMove) -> Board {
        // moves are assumed to be at least sensical
        let piece = self.board[chess_move.from_ind.0 as usize][chess_move.from_ind.1 as usize];
        self.board[chess_move.from_ind.0 as usize][chess_move.from_ind.1 as usize] = EMPTY;
        if chess_move.promotion == EMPTY {
            self.board[chess_move.to_ind.0 as usize][chess_move.to_ind.1 as usize] = piece;
        } else {
            // promotion piece doesn't have team info
            self.board[chess_move.to_ind.0 as usize][chess_move.to_ind.1 as usize] = (chess_move.promotion & UNIT_MASK) | (piece & TEAM_MASK);
        }

        // check castling moves
        if chess_move.from_ind.0 == 4 && chess_move.from_ind.1 == 0 {
            if chess_move.to_ind.0 == 6 && chess_move.to_ind.1 == 0 {
                // white king side castle, move the rook
                self.board[7][0] = EMPTY;
                self.board[5][0] = TEAM_WHITE | ROOK;
                self.white_ks_castle = false;
                self.white_qs_castle = false;
            }
            if chess_move.to_ind.0 == 2 && chess_move.to_ind.1 == 0 {
                // white king side castle, move the rook
                self.board[0][0] = EMPTY;
                self.board[3][0] = TEAM_WHITE | ROOK;
                self.white_ks_castle = false;
                self.white_qs_castle = false;
            }
        }
        if chess_move.from_ind.0 == 4 && chess_move.from_ind.1 == 7 {
            if chess_move.to_ind.0 == 6 && chess_move.to_ind.1 == 7 {
                // white king side castle, move the rook
                self.board[7][7] = EMPTY;
                self.board[5][7] = TEAM_WHITE | ROOK;
                self.black_ks_castle = false;
                self.black_qs_castle = false;
            }
            if chess_move.to_ind.0 == 2 && chess_move.to_ind.1 == 7 {
                // white king side castle, move the rook
                self.board[0][7] = EMPTY;
                self.board[3][7] = TEAM_WHITE | ROOK;
                self.black_ks_castle = false;
                self.black_qs_castle = false;
            }
        }

        // toggle move
        self.is_whites_move = !self.is_whites_move;

        // set move times
        self.half_move_clock += 1;
        self.full_move_clock = (self.half_move_clock / 2) + 1;

        // check if white or black are in check
        let white_moves = self.possible_moves_for(TEAM_WHITE);
        let white_king_ind = self.find_piece(TEAM_WHITE | KING).unwrap_or((-1, -1));

        let black_moves = self.possible_moves_for(TEAM_BLACK);
        let black_king_ind = self.find_piece(TEAM_BLACK | KING).unwrap_or((-1, -1));

        self.white_in_check = black_moves.iter().any(|cm| cm.to_ind == white_king_ind);
        self.black_in_check = white_moves.iter().any(|cm| cm.to_ind == black_king_ind);

        self
    }

    /// Get a vector of all possible moves for the player whos turn it is.
    /// An empty vector implies being in checkmate.
    pub fn get_legal_moves(&self) -> Vec<ChessMove> {
        let team_mask = if self.is_whites_move { TEAM_WHITE } else { TEAM_BLACK };

        self.possible_moves_for(team_mask)
            .into_iter()
            .filter(|cm| self.move_is_legal(cm))
            .collect()
    }

    /// Get estimated score. Score is an float32, distributed around 0, advising whites advantage
    pub fn score(&self) -> f32 {
        let mut white_score = 0_f32;
        let mut black_score = 0_f32;

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.board[i][j];
                let ps = match UNIT_MASK & piece {
                    PAWN => 1_f32,
                    ROOK => 3.5_f32,
                    KNIGHT => 3_f32,
                    BISHOP => 2.5_f32,
                    QUEEN => 6_f32,
                    _ => 0_f32,
                };

                if TEAM_MASK & piece == TEAM_WHITE {
                    white_score += ps;
                } else {
                    black_score += ps;
                }
            }
        }

        return white_score - black_score
    }

    pub fn as_debug_string(&self) -> String {
        let mut msg = String::with_capacity(64 + 8 + 17);
        msg.push_str("\n<Board>\n");
        msg.push_str(format!("current turn: {}\n", if self.is_whites_move { "white" } else { "black" }).as_str());
        msg.push_str(format!("white in check:   {}\n", self.white_in_check).as_str());
        msg.push_str(format!("white can castle: {}\n", self.white_qs_castle || self.white_ks_castle).as_str());
        msg.push_str(format!("black in check:   {}\n", self.black_in_check).as_str());
        msg.push_str(format!("black can castle: {}\n", self.black_qs_castle || self.black_ks_castle).as_str());
        msg.push_str(format!("board score:      {}\n", self.score()).as_str());
        for i in (0..8).rev() {
            msg.push_str(format!("{} ", i + 1).as_str());
            for j in 0..8 {
                msg.push(char_for_slot(self.board[j][i]))
            }
            msg.push('\n')
        }
        msg.push_str("  abcdefgh\n</Board>");

        return msg
    }

    #[cfg(test)]
    pub fn as_test_string(&self) -> String {
        let mut msg = String::with_capacity(64 + 8);
        for row in self.board.iter().rev() {
            for slot in row {
                msg.push(char_for_slot(*slot))
            }
            msg.push('/')
        };

        return msg
    }

    //
    // Util
    //

    fn piece_at(&self, pos: (i32, i32)) -> u8 {
        self.board[pos.0 as usize][pos.1 as usize]
    }

    fn team_owns_piece(team: u8, piece: u8) -> bool {
        ((TEAM_MASK & piece) ^ team == 0) && (piece & UNIT_MASK > 0)
    }

    fn possible_moves_for(&self, team_mask: u8) -> Vec<ChessMove> {
        let mut possible_moves: Vec<ChessMove> = Vec::with_capacity(5); // Total guess

        for x in 0..8 {
            for y in 0..8 {
                let slot = self.board[x][y];
                if Board::team_owns_piece(team_mask, slot) {
                    possible_moves.append(&mut self.get_possible_moves_for_piece(x as i32, y as i32))
                }
            }
        };

        possible_moves
    }

    // TODO just use a buffer and remove Vec for faster non-heap access?
    fn get_possible_moves_for_piece(&self, x: i32, y: i32) -> Vec<ChessMove> {
        let piece = self.board[x as usize][y as usize];

        return if piece & UNIT_MASK == PAWN {
            // have to consider team for direction, and the possible first double move, and possible promotion
            // TODO pawn promotion
            let mut res: Vec<Option<ChessMove>> = Vec::with_capacity(4);
            if piece & TEAM_MASK == TEAM_WHITE {
                // add the possible push move
                let m1 = ChessMove::from_delta(x, y, 0, 1)
                    .filter(|cm| !self.move_is_take(cm));

                // if the first move is allowed, check for the double push
                if m1.is_some() && y == 1 {
                    res.push(ChessMove::from_delta(x, y, 0, 2)
                                 .filter(|cm| !self.move_is_take(cm)));
                }

                // add in the first move
                res.push(m1);

                // add in diagonal attacks
                ChessMove::from_delta(x, y, -1, 1)
                    .filter(|cm| self.move_is_possible(cm))
                    .filter(|cm| self.move_is_take(cm))
                    .map(|cm| res.push(Some(cm)));
                ChessMove::from_delta(x, y, 1, 1)
                    .filter(|cm| self.move_is_possible(cm))
                    .filter(|cm| self.move_is_take(cm))
                    .map(|cm| res.push(Some(cm)));


            } else {
                // add the possible push move
                let m1 = ChessMove::from_delta(x, y, 0, -1)
                    .filter(|cm| !self.move_is_take(cm));

                // if the first move is allowed, check for the double push
                if m1.is_some() && y == 1 {
                    res.push(ChessMove::from_delta(x, y, 0, -2)
                        .filter(|cm| !self.move_is_take(cm)));
                }

                // add in the first move
                res.push(m1);

                // add in diagonal attacks
                ChessMove::from_delta(x, y, -1, -1)
                    .filter(|cm| self.move_is_possible(cm))
                    .filter(|cm| self.move_is_take(cm))
                    .map(|cm| res.push(Some(cm)));
                ChessMove::from_delta(x, y, 1, -1)
                    .filter(|cm| self.move_is_possible(cm))
                    .filter(|cm| self.move_is_take(cm))
                    .map(|cm| res.push(Some(cm)));
            }

            res
                .into_iter()
                .filter(|cm| cm.is_some())
                .map(|cm| cm.unwrap())
                .filter(|cm| self.move_is_possible(cm))
                .collect()
        } else if piece & UNIT_MASK == KING {
            // TODO castling LOL
            vec![
                ChessMove::from_delta(x, y, 0, 1),
                ChessMove::from_delta(x, y, 1, 1),
                ChessMove::from_delta(x, y, 1, 0),
                ChessMove::from_delta(x, y, 1, -1),
                ChessMove::from_delta(x, y, 0, -1),
                ChessMove::from_delta(x, y, -1, -1),
                ChessMove::from_delta(x, y, -1, 0),
                ChessMove::from_delta(x, y, -1, 1),
            ]
                .into_iter()
                .filter(|cm| cm.is_some())
                .map(|cm| cm.unwrap())
                .filter(|cm| self.move_is_possible(cm))
                .collect()
        } else if piece & UNIT_MASK == ROOK {
            // up down left right
            let mut res: Vec<ChessMove> = Vec::with_capacity(14);
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, 0, i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, 0));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, 0, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, 0));
            res
        } else if piece & UNIT_MASK == BISHOP {
            // diagonals baby
            let mut res: Vec<ChessMove> = Vec::with_capacity(13);
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, i));
            res
        } else if piece & UNIT_MASK == KNIGHT {
            // oh no
            vec![
                ChessMove::from_delta(x, y, -1, 2),
                ChessMove::from_delta(x, y, 1, 2),
                ChessMove::from_delta(x, y, -1, -2),
                ChessMove::from_delta(x, y, 1, -2),
                ChessMove::from_delta(x, y, 2, 1),
                ChessMove::from_delta(x, y, 2, -1),
                ChessMove::from_delta(x, y, -2, 1),
                ChessMove::from_delta(x, y, -2, -1),
            ]
                .into_iter()
                .filter(|cm| cm.is_some())
                .map(|cm| cm.unwrap())
                .filter(|cm| self.move_is_possible(cm))
                .collect()
        } else if piece & UNIT_MASK == QUEEN {
            // up down left right diagonals!
            let mut res: Vec<ChessMove> = Vec::with_capacity(27);
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, 0, i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, 0));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, i, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, 0, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, -i));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, 0));
            self.possible_move_line(&mut res, |i| ChessMove::from_delta(x, y, -i, i));
            res
        } else {
            vec![]
        };

        // res.into_iter()
        //     .filter(|&cm| cm.is_some())
        //     .map(|cm| cm.unwrap())
        //     .filter(|cm| self.move_is_possible(cm))
        //     .collect()
    }

    /// tries to find the location of a piece on the board
    fn find_piece(&self, piece: u8) -> Option<(i32, i32)> {
        for i in 0..8 {
            for j in 0..8 {
                if self.piece_at((i, j)) == piece {
                    return Some((i, j))
                }
            }
        };
        None
    }

    /// This is a step-1 check. It just makes sure the move is to an empty tile, or an enemy tile.
    /// It _doesn't_ ensure that the move doesn't put the maker in check.
    fn move_is_possible(&self, cm: &ChessMove) -> bool {
        // check to make sure the slot moving too isn't occupied
        let from_slot = self.piece_at(cm.from_ind);
        let to_slot = self.piece_at(cm.to_ind);

        return to_slot == EMPTY || (from_slot & TEAM_MASK != to_slot & TEAM_MASK)
    }

    /// returns true if this move would take a piece
    /// the move is assumed to already have been checked by move_is_possible
    fn move_is_take(&self, cm: &ChessMove) -> bool {
        let to_slot = self.piece_at(cm.to_ind);
        return to_slot != EMPTY
    }

    /// Returns true if the move is actually legal. This means that
    ///    1) it doesn't put yourself in check
    ///    2) if you're already in check, it gets you out of check
    fn move_is_legal(&self, cm: &ChessMove) -> bool {
        // apply the move and get a new board
        let new_board: Board = self.clone().apply_move(cm);

        // basically you can't end your turn in check; this covers both cases
        let team_mask = self.piece_at(cm.from_ind) & TEAM_MASK;
        if team_mask == TEAM_WHITE {
            !new_board.white_in_check
        } else {
            !new_board.black_in_check
        }
    }

    /// help rip a line of possible moves. puts any moves that pass into the res Vec
    fn possible_move_line<F>(&self, res: &mut Vec<ChessMove>, func: F)
        where F: Fn(i32) -> Option<ChessMove> {
        for i in 1..8 {
            let cm = func(i).filter(|cm| self.move_is_possible(cm));
            if cm.is_some() {
                // res.push(cm.unwrap());
                let take_cm = cm.clone().filter(|cm| self.move_is_take(cm));
                if take_cm.is_some() {
                    res.push(cm.unwrap());
                    break
                } else {
                    res.push(cm.unwrap());
                }
            } else {
                break
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_empty_new() {
        assert_eq!(
            Board::new().as_test_string(),
            "        /        /        /        /        /        /        /        /",
        )
    }

    #[test]
    fn test_reset_from_fen_startpos() {
        let mut board = Board::new();

        board
            .reset_from_fen(["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"])
            .unwrap();
        assert_eq!(
            board.as_test_string(),
            "rnbqkbnr/pppppppp/        /        /        /        /PPPPPPPP/RNBQKBNR/"
        );
        assert_eq!(board.is_whites_move, true);
        assert_eq!(board.white_ks_castle, true);
        assert_eq!(board.white_qs_castle, true);
        assert_eq!(board.black_ks_castle, true);
        assert_eq!(board.black_qs_castle, true);
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_clock, 1);

        board
            .reset_from_fen(["rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", "w", "Kq", "-", "100", "8"])
            .unwrap();
        assert_eq!(
            board.as_test_string(),
            "rnbqkbnr/pppppppp/        /        /    P   /        /PPPP PPP/RNBQKBNR/"
        );
        assert_eq!(board.is_whites_move, true);
        assert_eq!(board.white_ks_castle, true);
        assert_eq!(board.white_qs_castle, false);
        assert_eq!(board.black_ks_castle, false);
        assert_eq!(board.black_qs_castle, true);
        assert_eq!(board.half_move_clock, 100);
        assert_eq!(board.full_move_clock, 8);

        board
            .reset_from_fen(["rnbqkbnr/p1p1p1p1/1p1p1p1p/8/4P3/8/PPPP1PPP/RNBQKBNR", "b", "-", "-", "100", "8"])
            .unwrap();
        assert_eq!(
            board.as_test_string(),
            "rnbqkbnr/p p p p / p p p p/        /    P   /        /PPPP PPP/RNBQKBNR/"
        );
        assert_eq!(board.is_whites_move, false);
        assert_eq!(board.white_ks_castle, false);
        assert_eq!(board.white_qs_castle, false);
        assert_eq!(board.black_ks_castle, false);
        assert_eq!(board.black_qs_castle, false);
        assert_eq!(board.half_move_clock, 100);
        assert_eq!(board.full_move_clock, 8);
    }
}



// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     }
// }

