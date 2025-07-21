use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
const BLOCKSIZE: f32 = 75.0;
const SPRITESIZE: f32 = 45.0;
const BUFF: f32 = 2.0;
const MAX_POSSIBLE_LEGAL_MOVES: usize = 32; // technically 28 but rounding off to nearest two powers

#[derive(Clone, Copy, PartialEq, Eq,Debug)]
enum PieceType {
    WK,
    BK,
    W,
    B,
    WQ,
    BQ,
    WR,
    BR,
    WN,
    BN,
    WB,
    BB,
    E,
}

enum GameCondition{
    Running,
    Draw,
    WhiteWin,
    BlackWin,
    StartScreen,
    Quit,
    Restart
}

use PieceType::*;

fn draw_piece(spritesheet: &Texture2D, piecetype: &PieceType, posx: usize, posy: usize) {
    let (coordinate_x, coordinate_y) = match piecetype {
        WK => (0.0, 0.0),
        BK => (0.0, 1.0),
        WQ => (1.0, 0.0),
        BQ => (1.0, 1.0),
        WB => (2.0, 0.0),
        BB => (2.0, 1.0),
        WN => (3.0, 0.0),
        BN => (3.0, 1.0),
        WR => (4.0, 0.0),
        BR => (4.0, 1.0),
        W => (5.0, 0.0),
        B => (5.0, 1.0),
        E => (6.0, 6.0),
    };
    if coordinate_x != 6.0 {
        draw_texture_ex(
            spritesheet,
            posx as f32 * BLOCKSIZE,
            posy as f32 * BLOCKSIZE,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    coordinate_x * SPRITESIZE,
                    coordinate_y * SPRITESIZE,
                    SPRITESIZE,
                    SPRITESIZE,
                )),
                dest_size: Some(Vec2::new(BLOCKSIZE, BLOCKSIZE)),
                ..Default::default()
            },
        );
    }
}

struct Board{
    width: usize,
    height: usize,
    pieces: Vec<Vec<PieceType>>,
    spritesheet: Texture2D,
    white_king_x: usize,
    white_king_y: usize,
    black_king_x: usize,
    black_king_y: usize,
    is_white_king_moved:bool,
    is_black_king_moved:bool,
    is_left_white_rook_moved:bool,
    is_right_white_rook_moved:bool,
    is_left_black_rook_moved:bool,
    is_right_black_rook_moved:bool
}

impl Board {
    fn new(sprite_sheet: Texture2D) -> Self{
        Board {
            width: 8,
            height: 8,
            spritesheet: sprite_sheet,
            pieces: vec![
                vec![BR, BN, BB, BQ, BK, BB, BN, BR],
                vec![B, B, B, B, B, B, B, B],
                vec![E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E, E],
                vec![W, W, W, W, W, W, W, W],
                vec![WR, WN, WB, WQ, WK, WB, WN, WR],
            ],
            white_king_x: 4,
            white_king_y: 7,
            black_king_x: 4,
            black_king_y: 0,
            is_white_king_moved:false,
            is_black_king_moved:false,
            is_left_white_rook_moved:false,
            is_right_white_rook_moved:false,
            is_left_black_rook_moved:false,
            is_right_black_rook_moved:false
            
        }
    }
    fn draw(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let color = match (i + j) % 2 {
                    0 => BLUE,
                    1 => SKYBLUE,
                    _ => BLUE,
                };
                draw_rectangle(
                    i as f32 * BLOCKSIZE,
                    j as f32 * BLOCKSIZE,
                    BLOCKSIZE,
                    BLOCKSIZE,
                    color,
                );
                draw_piece(&self.spritesheet, &self.pieces[j][i], i, j);
            }
        }
    }
    fn is_black_piece(&self, x: usize, y: usize) -> bool {
        matches!(self.pieces[y][x], B | BK | BQ | BR | BN | BB)
    }

    fn is_white_piece(&self, x: usize, y: usize) -> bool {
        matches!(self.pieces[y][x], W | WK | WQ | WR | WN | WB)
    }

    fn is_white_king_in_check(&self, x: usize, y: usize) -> bool {
        // straight line check
        for i in (0..x).rev() {
            if self.pieces[y][i] != E {
                if self.pieces[y][i] == BR || self.pieces[y][i] == BQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in x + 1..self.width {
            if self.pieces[y][i] != E {
                if self.pieces[y][i] == BR || self.pieces[y][i] == BQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in (0..y).rev() {
            if self.pieces[i][x] != E {
                if self.pieces[i][x] == BR || self.pieces[i][x] == BQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in y + 1..self.height {
            if self.pieces[i][x] != E {
                if self.pieces[i][x] == BR || self.pieces[i][x] == BQ {
                    return true;
                } else {
                    break;
                }
            }
        }

        // diagonal cheking

        let mut curr_x = (x + 1) as i32;
        let mut curr_y = (y + 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == BB
                || self.pieces[curr_y as usize][curr_x as usize] == BQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x += 1;
            curr_y += 1;
        }
        let mut curr_x = (x - 1) as i32;
        let mut curr_y = (y + 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == BB
                || self.pieces[curr_y as usize][curr_x as usize] == BQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x -= 1;
            curr_y += 1;
        }
        let mut curr_x = (x + 1) as i32;
        let mut curr_y = (y - 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == BB
                || self.pieces[curr_y as usize][curr_x as usize] == BQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x += 1;
            curr_y -= 1;
        }
        let mut curr_x = (x - 1) as i32;
        let mut curr_y = (y - 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == BB
                || self.pieces[curr_y as usize][curr_x as usize] == BQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x -= 1;
            curr_y -= 1;
        }

        // for the knight

        let diffs = [
            (2, 1),
            (-2, 1),
            (2, -1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
        ];
        let curr_x = x as i32;
        let curr_y = y as i32;

        for diff in diffs {
            if curr_x + diff.0 < self.width as i32
            && curr_x + diff.0 >= 0
            && curr_y + diff.1 < self.height as i32
            && curr_y + diff.1 >= 0
            && self.pieces[(curr_y + diff.1) as usize][(curr_x + diff.0) as usize] == BN
            {
                return true;
            }
        }

        // king check
        let diffs = [
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (0, 1),
        ];

        for diff in diffs {
            if curr_x + diff.0 < self.width as i32
            && curr_x + diff.0 >= 0
            && curr_y + diff.1 < self.height as i32
            && curr_y + diff.1 >= 0
            && self.pieces[(curr_y + diff.1) as usize][(curr_x + diff.0) as usize] == BK
            {
                return true;
            }
        }

        // pawn check

        if (curr_y - 1) >= 0
        && (((curr_x - 1) >= 0
        && self.pieces[(curr_y - 1) as usize][(curr_x - 1) as usize] == B)
        || curr_x + 1 < self.width as i32
        && self.pieces[(curr_y - 1) as usize][(curr_x + 1) as usize] == B)
        {
            return true;
        }
        false
    }
    fn is_black_king_in_check(&self, x: usize, y: usize) -> bool {
        // straight line check
        for i in (0..x).rev() {
            if self.pieces[y][i] != E {
                if self.pieces[y][i] == WR || self.pieces[y][i] == WQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in x + 1..self.width {
            if self.pieces[y][i] != E {
                if self.pieces[y][i] == WR || self.pieces[y][i] == WQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in (0..y).rev() {
            if self.pieces[i][x] != E {
                if self.pieces[i][x] == WR || self.pieces[i][x] == WQ {
                    return true;
                } else {
                    break;
                }
            }
        }
        for i in y + 1..self.height {
            if self.pieces[i][x] != E {
                if self.pieces[i][x] == WR || self.pieces[i][x] == WQ {
                    return true;
                } else {
                    break;
                }
            }
        }

        // diagonal cheking

        let mut curr_x = (x + 1) as i32;
        let mut curr_y = (y + 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == WB
                || self.pieces[curr_y as usize][curr_x as usize] == WQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x += 1;
            curr_y += 1;
        }
        let mut curr_x = x as i32 - 1;
        let mut curr_y = (y + 1) as i32;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == WB
                || self.pieces[curr_y as usize][curr_x as usize] == WQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x -= 1;
            curr_y += 1;
        }
        let mut curr_x = (x + 1) as i32;
        let mut curr_y = y as i32 - 1;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == WB
                || self.pieces[curr_y as usize][curr_x as usize] == WQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x += 1;
            curr_y -= 1;
        }
        let mut curr_x = x as i32 - 1;
        let mut curr_y = y as i32 - 1;

        while curr_x < (self.width as i32)
        && curr_y < (self.height as i32)
        && curr_y >= 0
        && curr_x >= 0
        {
            if self.pieces[curr_y as usize][curr_x as usize] != E {
                if self.pieces[curr_y as usize][curr_x as usize] == WB
                || self.pieces[curr_y as usize][curr_x as usize] == WQ
                {
                    return true;
                } else {
                    break;
                }
            }
            curr_x -= 1;
            curr_y -= 1;
        }

        // for the knight

        let diffs = [
            (2, 1),
            (-2, 1),
            (2, -1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
        ];
        let curr_x = x as i32;
        let curr_y = y as i32;

        for diff in diffs {
            if curr_x + diff.0 < self.width as i32
            && curr_x + diff.0 >= 0
            && curr_y + diff.1 < self.height as i32
            && curr_y + diff.1 >= 0
            && self.pieces[(curr_y + diff.1) as usize][(curr_x + diff.0) as usize] == WN
            {
                return true;
            }
        }

        // king check
        let diffs = [
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (0, 1),
        ];

        for diff in diffs {
            if curr_x + diff.0 < self.width as i32
            && curr_x + diff.0 >= 0
            && curr_y + diff.1 < self.height as i32
            && curr_y + diff.1 >= 0
            && self.pieces[(curr_y + diff.1) as usize][(curr_x + diff.0) as usize] == WK
            {
                return true;
            }
        }

        // pawn check

        if (curr_y + 1) < self.height as i32
        && (((curr_x - 1) >= 0
        && self.pieces[(curr_y + 1) as usize][(curr_x - 1) as usize] == W)
        || (curr_x + 1 < self.width as i32
        && self.pieces[(curr_y + 1) as usize][(curr_x + 1) as usize] == W))
        {
            return true;
        }
        false
    }

    fn is_legal_move(&mut self, src_x: usize, src_y: usize, dist_x: usize, dist_y: usize) -> bool {
        let removed_piece = self.pieces[dist_y][dist_x];
        let current_piece = self.pieces[src_y][src_x];

        if self.is_black_piece(src_x, src_y) && self.is_black_piece(dist_x, dist_y) {
            return false;
        }
        if self.is_white_piece(src_x, src_y) && self.is_white_piece(dist_x, dist_y) {
            return false;
        }

        if self.is_black_piece(src_x, src_y) {
            self.pieces[dist_y][dist_x] = current_piece;
            self.pieces[src_y][src_x] = E;
            if current_piece == BK {
                self.black_king_x = dist_x;
                self.black_king_y = dist_y;
            }

            if self.is_black_king_in_check(self.black_king_x, self.black_king_y) {
                self.pieces[src_y][src_x] = current_piece;
                self.pieces[dist_y][dist_x] = removed_piece;
                if current_piece == BK {
                    self.black_king_x = src_x;
                    self.black_king_y = src_y;
                }
                return false;
            }
            if current_piece == BK {
                self.black_king_x = src_x;
                self.black_king_y = src_y;
            }
        }
        if self.is_white_piece(src_x, src_y) {
            self.pieces[dist_y][dist_x] = current_piece;
            self.pieces[src_y][src_x] = E;
            if current_piece == WK {
                self.white_king_x = dist_x;
                self.white_king_y = dist_y;
            }

            if self.is_white_king_in_check(self.white_king_x, self.white_king_y) {
                self.pieces[src_y][src_x] = current_piece;
                self.pieces[dist_y][dist_x] = removed_piece;
                if current_piece == WK {
                    self.white_king_x = src_x;
                    self.white_king_y = src_y;
                }
                return false;
            }
            if current_piece == WK {
                self.white_king_x = src_x;
                self.white_king_y = src_y;
            }
        }
        self.pieces[src_y][src_x] = current_piece;
        self.pieces[dist_y][dist_x] = removed_piece;

        true
    }

    fn get_legal_moves_for_piece(
        &mut self,
        src_x: usize,
        src_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut legal_moves: Vec<(usize, usize)> = Vec::with_capacity(MAX_POSSIBLE_LEGAL_MOVES);
        let curr_x = src_x as i32;
        let curr_y = src_y as i32;
        match self.pieces[src_y][src_x] {
            WK | BK => {
                let diffs = [
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                    (1, 0),
                    (-1, 0),
                    (0, -1),
                    (0, 1),
                ];

                for diff in diffs {
                    let dest_x: i32 = curr_x + diff.0;
                    let dest_y: i32 = curr_y + diff.1;
                    if dest_x < self.width as i32
                    && dest_x >= 0
                    && dest_y < self.height as i32
                    && dest_y >= 0
                    && self.is_legal_move(
                        curr_x as usize,
                        curr_y as usize,
                        dest_x as usize,
                        dest_y as usize,
                    )
                    {
                        legal_moves.push((dest_x as usize, dest_y as usize));
                    }
                }

                if self.pieces[src_y][src_x] == WK &&
                src_y == self.width-1 && src_x == 4 &&
                    !self.is_white_king_moved{
                        
                    if !self.is_right_white_rook_moved &&
                        self.pieces[src_y][5]==E&&
                        self.pieces[src_y][6]==E &&
        self.is_legal_move(src_x, src_y, 6, src_y)&&
        self.is_legal_move(src_x, src_y, 5, src_y){
                            legal_moves.push((6,src_y)) ;
                    }
                    if !self.is_left_white_rook_moved  &&
                        self.pieces[src_y][3] ==E &&
                        self.pieces[src_y][2] ==E &&
                        self.is_legal_move(src_x, src_y, 3, src_y)&&
                        self.is_legal_move(src_x, src_y, 2, src_y){
                            legal_moves.push((2,src_y)) ;
                    }
                    
                }
                if self.pieces[src_y][src_x]==BK&&
                src_y == 0 && src_x == 4 &&
                    !self.is_black_king_moved{
                    if !self.is_right_black_rook_moved &&
                        self.pieces[src_y][5]==E&&
                        self.pieces[src_y][6]==E &&
        self.is_legal_move(src_x, src_y, 6, src_y)&&
        self.is_legal_move(src_x, src_y, 5, src_y){
                            legal_moves.push((6,src_y)) ;
                    }
                    if !self.is_left_black_rook_moved  &&
                        self.pieces[src_y][3] ==E &&
                        self.pieces[src_y][2] ==E &&
                        self.is_legal_move(src_x, src_y, 3, src_y)&&
                        self.is_legal_move(src_x, src_y, 2, src_y){
                            legal_moves.push((2,src_y)) ;
                    }

                }
            }
            WN | BN => {
                let diffs = [
                    (2, 1),
                    (-2, 1),
                    (2, -1),
                    (-2, -1),
                    (1, 2),
                    (-1, 2),
                    (1, -2),
                    (-1, -2),
                ];

                for diff in diffs {
                    let dest_x: i32 = curr_x + diff.0;
                    let dest_y: i32 = curr_y + diff.1;
                    if dest_x < self.width as i32
                    && dest_x >= 0
                    && dest_y < self.height as i32
                    && dest_y >= 0
                    && self.is_legal_move(
                        curr_x as usize,
                        curr_y as usize,
                        dest_x as usize,
                        dest_y as usize,
                    )
                    {
                        legal_moves.push((dest_x as usize, dest_y as usize));
                    }
                }
            }
            WR | BR => {
                for i in (0..src_x).rev() {
                    if self.is_legal_move(src_x, src_y, i, src_y) {
                        legal_moves.push((i, src_y));
                    }
                    if self.pieces[src_y][i] != E {
                        break;
                    }
                }
                for i in src_y + 1..self.width {
                    if self.is_legal_move(src_x, src_y, i, src_y) {
                        legal_moves.push((i, src_y));
                    }
                    if self.pieces[src_y][i] != E {
                        break;
                    }
                }
                for j in (0..src_y).rev() {
                    if self.is_legal_move(src_x, src_y, src_x, j) {
                        legal_moves.push((src_x, j));
                    }
                    if self.pieces[j][src_x] != E {
                        break;
                    }
                }
                for j in src_y + 1..self.width {
                    if self.is_legal_move(src_x, src_y, src_x, j) {
                        legal_moves.push((src_x, j));
                    }
                    if self.pieces[j][src_x] != E {
                        break;
                    }
                }
            }
            WB | BB => {
                let move_dirs = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

                for move_dir in move_dirs {
                    let mut dest_x = src_x as i32 + move_dir.0;
                    let mut dest_y = src_y as i32 + move_dir.1;
                    while dest_x >= 0
                    && dest_x < self.width as i32
                    && dest_y >= 0
                    && dest_y < self.height as i32
                    {
                        if self.is_legal_move(src_x, src_y, dest_x as usize, dest_y as usize) {
                            legal_moves.push((dest_x as usize, dest_y as usize));
                        }
                        if self.pieces[dest_y as usize][dest_x as usize] != E {
                            break;
                        }
                        dest_x += move_dir.0;
                        dest_y += move_dir.1;
                    }
                }
            }
            WQ | BQ => {
                for i in (0..src_x).rev() {
                    if self.is_legal_move(src_x, src_y, i, src_y) {
                        legal_moves.push((i, src_y));
                    }
                    if self.pieces[src_y][i] != E {
                        break;
                    }
                }
                for i in src_x + 1..self.width {
                    if self.is_legal_move(src_x, src_y, i, src_y) {
                        legal_moves.push((i, src_y));
                    }
                    if self.pieces[src_y][i] != E {
                        break;
                    }
                }
                for j in (0..src_y).rev() {
                    if self.is_legal_move(src_x, src_y, src_x, j) {
                        legal_moves.push((src_x, j));
                    }
                    if self.pieces[j][src_x] != E {
                        break;
                    }
                }
                for j in src_y + 1..self.width {
                    if self.is_legal_move(src_x, src_y, src_x, j) {
                        legal_moves.push((src_x, j));
                    }
                    if self.pieces[j][src_x] != E {
                        break;
                    }
                }

                let move_dirs = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

                for move_dir in move_dirs {
                    let mut dest_x = src_x as i32 + move_dir.0;
                    let mut dest_y = src_y as i32 + move_dir.1;
                    while dest_x >= 0
                    && dest_x < self.width as i32
                    && dest_y >= 0
                    && dest_y < self.height as i32
                    {
                        if self.is_legal_move(src_x, src_y, dest_x as usize, dest_y as usize) {
                            legal_moves.push((dest_x as usize, dest_y as usize));
                        }
                        if self.pieces[dest_y as usize][dest_x as usize] != E {
                            break;
                        }
                        dest_x += move_dir.0;
                        dest_y += move_dir.1;
                    }
                }
            }
            W => {
                // one move forward

                if self.pieces[src_y - 1][src_x] == E
                && self.is_legal_move(src_x, src_y, src_x, src_y-1)
                {
                    legal_moves.push((src_x, src_y - 1));
                }
                // two move forward
                if src_y == 6
                && self.pieces[src_y - 1][src_x] == E
                && self.pieces[src_y - 2][src_x] == E
                && self.is_legal_move(src_x, src_y, src_x, src_y-2)
                {
                    legal_moves.push((src_x, src_y - 2));
                }

                // capture

                if (curr_x - 1) >= 0
                && self.is_black_piece(src_x - 1, src_y - 1)
                && self.is_legal_move(src_x, src_y, src_x - 1, src_y - 1)
                {
                    legal_moves.push((src_x - 1, src_y - 1));
                }
                if (curr_x + 1) < self.width as i32
                && self.is_black_piece(src_x + 1, src_y - 1)
                && self.is_legal_move(src_x, src_y, src_x + 1, src_y - 1)
                {
                    legal_moves.push((src_x + 1, src_y - 1));
                }

                //TODO:: promotion
                // promotion
                // TODO: enpassant
                // enpassant
            }
            B => {
                if self.pieces[src_y + 1][src_x] == E
                && self.is_legal_move(src_x, src_y, src_x, src_y+1)
                {
                    legal_moves.push((src_x, src_y + 1));
                }
                // two move forward
                if src_y == 1
                && self.pieces[src_y + 1][src_x] == E
                && self.pieces[src_y + 2][src_x] == E
                && self.is_legal_move(src_x, src_y,src_x , src_y + 2)
                {
                    legal_moves.push((src_x, src_y + 2));
                }

                // capture

                if (curr_x - 1) >= 0
                && self.is_white_piece(src_x - 1, src_y + 1)
                && self.is_legal_move(src_x, src_y, src_x - 1, src_y + 1)
                {
                    legal_moves.push((src_x - 1, src_y + 1));
                }
                if (curr_x + 1) < self.width as i32
                && self.is_white_piece(src_x + 1, src_y + 1)
                && self.is_legal_move(src_x, src_y, src_x + 1, src_y + 1)
                {
                    legal_moves.push((src_x + 1, src_y + 1));
                }

                //TODO:: promotion
                // promotion
                // TODO: enpassant
                // enpassant
            }
            _ => {}
        }

        legal_moves
    }

    fn does_black_have_legal_moves(&mut self) -> bool{

        for i in 0..self.width{
            for j in 0..self.height{
                if self.is_black_piece(i,j){
                    let legal_moves = self.get_legal_moves_for_piece(i,j);
                    if !legal_moves.is_empty(){
                        return true;
                    }
                }
            }
        }
        false
    }
    fn does_white_have_legal_moves(&mut self) -> bool{

        for i in 0..self.width{
            for j in 0..self.height{
                if self.is_white_piece(i,j){
                    let legal_moves = self.get_legal_moves_for_piece(i,j);
                    if !legal_moves.is_empty(){
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn detect_mouse() -> (usize, usize) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        let i: usize = (x / BLOCKSIZE) as usize;
        let j: usize = (y / BLOCKSIZE) as usize;
        return (i, j);
    }
    (usize::MAX, usize::MAX)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Turn {
    White,
    Black,
}

struct Game {
    turn: Turn,
    board: Board,
    selected: bool,
    selected_x: usize,
    selected_y: usize,
    legal_moves: Vec<(usize, usize)>,
    game_condition:GameCondition
}

impl Game {
    fn new(spritesheet: Texture2D) -> Self {
        Game {
            turn: Turn::White,
            board: Board::new(spritesheet),
            selected: false,
            selected_x: 0,
            selected_y: 0,
            legal_moves: Vec::new(),
            game_condition:GameCondition::StartScreen
        }
    }

    fn deselect_and_clear_legal_moves(&mut self) {
        self.selected = false;
        self.legal_moves.clear();
    }
    fn change_selected_and_fetch_legal_moves(&mut self, x: usize, y: usize) {
        self.selected=true;
        self.selected_x = x;
        self.selected_y = y;

        self.legal_moves = self.board.get_legal_moves_for_piece(x, y);
    }
    fn move_piece(&mut self, x: usize, y: usize) {
        let mut selected_piece = self.board.pieces[self.selected_y][self.selected_x];
        // TODO: checking for promotion
        if selected_piece == W && y == 0 {
            selected_piece = WQ;
        } else if selected_piece == B && y == self.board.height - 1 {
            selected_piece = BQ;
        }

        if selected_piece == WK{
            if x == self.board.width -2 && y == self.board.height-1{
                self.board.pieces[y][x-1]=WR;
                self.board.pieces[y][x]=WK;
                self.board.pieces[y][x+1]=E;
            }
            if x == 2 && y == self.board.height-1{
                self.board.pieces[y][x+1]=WR;
                self.board.pieces[y][x]=WK;
                self.board.pieces[y][0]=E;
            }
        }
        if selected_piece == BK{
            if x == self.board.width -2 && y == 0{
                self.board.pieces[y][x-1]=BR;
                self.board.pieces[y][x]=BK;
                self.board.pieces[y][x+1]=E;
            }
            if x == 2 && y == 0{
                self.board.pieces[y][x+1]=BR;
                self.board.pieces[y][x]=BK;
                self.board.pieces[y][0]=E;
            }
        }

        match selected_piece{
            WK=>{
                self.board.is_white_king_moved=true;
                self.board.white_king_x = x;
                self.board.white_king_y = y;
            },
            BK=>{
                self.board.is_black_king_moved=true;
                self.board.black_king_x = x;
                self.board.black_king_y = y;
            },
            WR=>{
                if self.selected_x == 0{
                    self.board.is_left_white_rook_moved = true;
                }
                if self.selected_x == self.board.width -1{
                    self.board.is_right_white_rook_moved = true;
                }
            },
            BR=>{
                if self.selected_x == 0{
                    self.board.is_left_black_rook_moved = true;
                }
                if self.selected_x == self.board.width -1{
                    self.board.is_right_black_rook_moved = true;
                }
            },
            _=>{},
        }
        self.board.pieces[y][x] = selected_piece;
        self.board.pieces[self.selected_y][self.selected_x] = E;
    }

    fn highlight_legal_moves(&self) {
        for &(x, y) in &self.legal_moves {
            draw_rectangle(x as f32 * BLOCKSIZE, y as f32*BLOCKSIZE, BLOCKSIZE, BLOCKSIZE, Color::new(0.0, 0.0, 0.0, 0.2));
        }
    }
    fn run(&mut self) {

        let has_legal_moves = match self.turn{
            Turn::White=>self.board.does_white_have_legal_moves(),
            Turn::Black=>self.board.does_black_have_legal_moves()
        };

        if !has_legal_moves {
            match self.turn{
                Turn::White=>{
                    if self.board.is_white_king_in_check(self.board.white_king_x,self.board.white_king_y){
                        self.game_condition = GameCondition::BlackWin;
                    }
                    else{
                        self.game_condition = GameCondition::Draw;
                    }
                },
                Turn::Black=>{
                    if self.board.is_black_king_in_check(self.board.black_king_x,self.board.black_king_y){
                        self.game_condition = GameCondition::WhiteWin;
                    }
                    else{
                        self.game_condition = GameCondition::Draw;
                    }
                },
            } 
        }

        self.board.draw();

        if self.selected {
            draw_rectangle(
                self.selected_x as f32 * BLOCKSIZE - BUFF,
                self.selected_y as f32 * BLOCKSIZE - BUFF,
                BLOCKSIZE + 2.0 * BUFF,
                BLOCKSIZE + 2.0 * BUFF,
                Color::new(0.0, 0.0, 0.0, 0.2),
            );
        }

        self.highlight_legal_moves();

        let (x, y) = detect_mouse();
        if x != usize::MAX && y != usize::MAX && x < self.board.width && y < self.board.height {
            if !self.selected {
                if self.turn==Turn::White  && self.board.is_white_piece(x,y){
                    self.change_selected_and_fetch_legal_moves(x, y);
                }
                if self.turn==Turn::Black  && self.board.is_black_piece(x,y){
                    self.change_selected_and_fetch_legal_moves(x, y);
                }

            } else if x == self.selected_x && y == self.selected_y {
                self.deselect_and_clear_legal_moves();
            } else if (self.board.is_black_piece(x, y)
            && self.board.is_black_piece(self.selected_x, self.selected_y) &&
            self.turn == Turn::Black) ||
            (self.board.is_white_piece(x, y)
            && self.board.is_white_piece(self.selected_x, self.selected_y)&&
            self.turn == Turn::White){
                self.change_selected_and_fetch_legal_moves(x, y);
            } else {
                if self.legal_moves.contains(&(x, y)) {
                    self.move_piece(x, y);
                    self.turn = match self.turn{
                        Turn::White=>Turn::Black,
                        Turn::Black=>Turn::White
                    };
                }
                self.deselect_and_clear_legal_moves();
            }
        }
    }
    fn screen(&mut self){
        let win_size = vec2(400., 200.);
        let win_pos = vec2(
            (screen_width() - win_size.x) / 2.0,
            (screen_height() - win_size.y) / 2.0,
        );

        widgets::Window::new(hash!("screen"), win_pos, win_size)
            .label("Welcome to the Chess")
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                let center = |y| Vec2::new((win_size.x - 200.) / 2.0, y);

                let message = match self.game_condition{
                    GameCondition::StartScreen=>"welcome to chess",
                    GameCondition::Draw=>"Draw",
                    GameCondition::BlackWin=>"Black Won",
                    GameCondition::WhiteWin=>"White Won",
                    _=>"Error"
                };

                ui.label(center(40.), message);
                ui.separator();
                let first_button_text = match self.game_condition{
                    GameCondition::StartScreen=>"Start Game",
                    GameCondition::Draw=>"Back to Start Screen",
                    GameCondition::BlackWin=>"Back to Start Screen",
                    GameCondition::WhiteWin=>"Back to start screen",
                    _=>"Erro"
                };

                if ui.button(center(100.), first_button_text) {
                    match self.game_condition{
                        GameCondition::StartScreen=>{
                            self.game_condition=GameCondition::Running
                        },
                        _=>{
                            self.game_condition = GameCondition::Restart; 
                        }
                    };
                }
                if ui.button(center(120.0),"quit"){
                    self.game_condition = GameCondition::Quit;
                }
            });
    }
}

#[macroquad::main("Chess")]
async fn main() {
    let sprite_sheet = load_texture("./../assets/Chess_Pieces_Sprite.png")
        .await
        .unwrap();
    let mut game = Game::new(sprite_sheet.clone());
    loop {
        clear_background(BLACK);
        match game.game_condition{
            GameCondition::StartScreen=> game.screen(),
            GameCondition::Running=>game.run(),
            GameCondition::Draw =>{
                game.screen();
            }
            GameCondition::WhiteWin =>{
                game.screen();
            }
            GameCondition::BlackWin =>{
                game.screen();
            }
            GameCondition::Quit=>{
                break;
            }
            GameCondition::Restart=>{
                game = Game::new(sprite_sheet.clone());
            }
        }
        next_frame().await
    }
}
