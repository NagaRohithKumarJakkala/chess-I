use macroquad::prelude::*;
const BLOCKSIZE:f32 = 75.0;
const SPRITESIZE:f32 = 45.0;
const BUFF:f32 = 2.0;

#[derive(Clone,Copy,PartialEq,Eq)]
enum PieceType{
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
    E
}

use PieceType::*;

fn draw_piece(spritesheet:&Texture2D,piecetype:&PieceType,posx:usize,posy:usize){
    let (coordinate_x,coordinate_y) =match piecetype{
        WK=>{
            (0.0,0.0)
        },
        BK=>{
            (0.0,1.0)
        },
        WQ=>{
            (1.0,0.0)
        },
        BQ=>{
            (1.0,1.0)
        },
        WB=>{
            (2.0,0.0)
        },
        BB=>{
            (2.0,1.0)
        },
        WN=>{
            (3.0,0.0)
        },
        BN=>{
            (3.0,1.0)
        },
        WR=>{
            (4.0,0.0)
        },
        BR=>{
            (4.0,1.0)
        },
        W=>{
            (5.0,0.0)
        },
        B=>{
            (5.0,1.0)
        },
        E=>{(6.0,6.0)}
    };
    if coordinate_x != 6.0{
        draw_texture_ex(
            spritesheet,
            posx as f32*BLOCKSIZE,
            posy as f32*BLOCKSIZE,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(coordinate_x*SPRITESIZE,coordinate_y*SPRITESIZE,SPRITESIZE,SPRITESIZE)),
                dest_size: Some(Vec2::new(
                        BLOCKSIZE,
                        BLOCKSIZE
                )),
                ..Default::default()
            },
        );
    }
}

struct Board{
    width:usize,
    height:usize,
    pieces:Vec<Vec<PieceType>>,
    spritesheet:Texture2D
}


impl Board{
    fn new(sprite_sheet:Texture2D)->Self{
        Board{
            width:8,
            height:8,
            spritesheet:sprite_sheet,
            pieces:vec![
                vec![BR,BN,BB,BQ,BK,BB,BN,BR],
                vec![B,B,B,B,B,B,B,B],
                vec![E,E,E,E,E,E,E,E],
                vec![E,E,E,E,E,E,E,E],
                vec![E,E,E,E,E,E,E,E],
                vec![E,E,E,E,E,E,E,E],
                vec![W,W,W,W,W,W,W,W],
                vec![WR,WN,WB,WQ,WK,WB,WN,WR]
            ]
        }
    }
    fn draw(&self) {
        for i in 0..self.width{
            for j in 0..self.height{
                let color = match (i+j)%2 {
                    0 =>BLUE,
                    1 =>SKYBLUE,
                    _=> BLUE
                };
                draw_rectangle(i as f32 *BLOCKSIZE, j as f32 *BLOCKSIZE, BLOCKSIZE, BLOCKSIZE, color);
                draw_piece(&self.spritesheet,&self.pieces[j][i],i,j);

            }
        }
    }
    fn is_black_piece(&self,x:usize,y:usize)->bool{
        matches!( self.pieces[y][x],B|BK|BQ|BR|BN|BB)
    }

    fn is_white_piece(&self,x:usize,y:usize)->bool{
        matches!( self.pieces[y][x],W|WK|WQ|WR|WN|WB)
    }

}

fn detect_mouse()->(usize,usize){
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x,y)= mouse_position();
        let i:usize = (x/BLOCKSIZE) as usize;
        let j:usize = (y/BLOCKSIZE) as usize;
        return (i,j)
    }
    (usize::MAX,usize::MAX)
}

enum Turn{
    White,
    Black
}

struct Game{
    turn:Turn,
    board:Board,
    selected:bool,
    selected_x:usize,
    selected_y:usize
}

impl Game{

    fn is_reachable(&self,x:usize,y:usize)->bool{
        match self.board.pieces[self.selected_y][self.selected_x]{
            W=>{
                if ((self.selected_x>0 &&x == self.selected_x-1) ||
                    x == self.selected_x+1) && 
                    (self.selected_y>0 && y == self.selected_y-1) &&
                        self.board.is_black_piece(x,y){
                    return true;
                }
                if self.board.pieces[y][x]!=E{
                    return false;
                }
                if x == self.selected_x &&
                    self.selected_y>0 &&
                    y == self.selected_y-1{
                    return true;
                }
                if self.selected_y == self.board.height-2 &&
                    x== self.selected_x &&
                    self.selected_y>1 &&
                    y == self.selected_y-2 &&
                    self.board.pieces[y+1][x]==E{
                        return true;
                }
                false
            },
            B=>{
                if ((self.selected_x>0 && x == self.selected_x-1)||
                    x == self.selected_x+1) &&
                    y == self.selected_y+1 &&
                    self.board.is_white_piece(x,y){
                    return true;
                }
                if self.board.pieces[y][x]!=E{
                    return false;
                }
                if x == self.selected_x &&
                    y == self.selected_y+1{
                    return true;
                }
                if self.selected_y == 1 &&
                    x == self.selected_x &&
                    y == self.selected_y+2 &&
                    self.board.pieces[y-1][x]==E{
                        return true;
                }
                false
            },
            WR=>{
                if self.board.is_white_piece(x,y){
                    return false;
                }
                else if x== self.selected_x {
                    for j in (y+1)..self.selected_y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }

                    for j in (self.selected_y+1)..y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if y== self.selected_y {
                    for i in (x+1)..self.selected_x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }

                    for i in (self.selected_x+1)..x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                false 
            },
            BR=>{
                if self.board.is_black_piece(x,y){
                    return false;
                }
                else if x== self.selected_x {
                    for j in (y+1)..self.selected_y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }

                    for j in (self.selected_y+1)..y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if y== self.selected_y {
                    for i in (x+1)..self.selected_x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }

                    for i in (self.selected_x+1)..x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                false 
            },
            WB=>{
                if self.board.is_white_piece(x,y){
                    return false;
                }
                else if (x as i32-self.selected_x as i32).abs() == (y as i32 - self.selected_y as i32).abs(){

                    let inc_x = (x as i32 - self.selected_x as i32).abs()/(x as i32 - self.selected_x as i32);
                    let inc_y = (y as i32 - self.selected_y as i32).abs()/(y as i32 - self.selected_y as i32);
                    let mut curr_x = self.selected_x as i32+inc_x;
                    let mut curr_y = self.selected_y as i32+inc_y;

                    while curr_x!=x as i32&& curr_y!=y as i32{
                        if self.board.pieces[curr_y as usize][curr_x as usize] !=E{
                            return false;
                        }
                        curr_x+=inc_x;
                        curr_y+=inc_y;
                    }
                    return true;
                }
                false
            },
            BB=>{
                if self.board.is_black_piece(x,y){
                    return false;
                }
                else if (x as i32-self.selected_x as i32).abs() == (y as i32 - self.selected_y as i32).abs(){

                    let inc_x = (x as i32 - self.selected_x as i32).abs()/(x as i32 - self.selected_x as i32);
                    let inc_y = (y as i32 - self.selected_y as i32).abs()/(y as i32 - self.selected_y as i32);
                    let mut curr_x = self.selected_x as i32+inc_x;
                    let mut curr_y = self.selected_y as i32+inc_y;

                    while curr_x!=x as i32 && curr_y!=y as i32{
                        if self.board.pieces[curr_y as usize][curr_x as usize] !=E{
                            return false;
                        }
                        curr_x+=inc_x;
                        curr_y+=inc_y;
                    }
                    return true;
                }
                false
            },
            WQ=>{
                if self.board.is_white_piece(x,y){
                    return false;
                }
                else if x== self.selected_x {
                    for j in (y+1)..self.selected_y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }

                    for j in (self.selected_y+1)..y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if y== self.selected_y {
                    for i in (x+1)..self.selected_x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }

                    for i in (self.selected_x+1)..x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if (x as i32-self.selected_x as i32).abs() == (y as i32 - self.selected_y as i32).abs(){
                    let inc_x = (x as i32 - self.selected_x as i32).abs()/(x as i32 - self.selected_x as i32);
                    let inc_y = (y as i32 - self.selected_y as i32).abs()/(y as i32 - self.selected_y as i32);
                    let mut curr_x = self.selected_x as i32+inc_x;
                    let mut curr_y = self.selected_y as i32+inc_y;

                    while curr_x!=x as i32&& curr_y!=y as i32{
                        if self.board.pieces[curr_y as usize][curr_x as usize] !=E{
                            return false;
                        }
                        curr_x+=inc_x;
                        curr_y+=inc_y;
                    }
                    return true;
                }
                false 
            },
            BQ=>{
                if self.board.is_black_piece(x,y){
                    return false;
                }
                else if x== self.selected_x {
                    for j in (y+1)..self.selected_y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }

                    for j in (self.selected_y+1)..y{
                        if self.board.pieces[j][x]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if y== self.selected_y {
                    for i in (x+1)..self.selected_x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }

                    for i in (self.selected_x+1)..x{
                        if self.board.pieces[y][i]!= E{
                            return false;
                        }
                    }
                    return true;
                }
                else if (x as i32-self.selected_x as i32).abs() == (y as i32 - self.selected_y as i32).abs(){
                    let inc_x = (x as i32 - self.selected_x as i32).abs()/(x as i32 - self.selected_x as i32);
                    let inc_y = (y as i32 - self.selected_y as i32).abs()/(y as i32 - self.selected_y as i32);
                    let mut curr_x = self.selected_x as i32+inc_x;
                    let mut curr_y = self.selected_y as i32+inc_y;

                    while curr_x!=x as i32&& curr_y!=y as i32{
                        if self.board.pieces[curr_y as usize][curr_x as usize] !=E{
                            return false;
                        }
                        curr_x+=inc_x;
                        curr_y+=inc_y;
                    }
                    return true;
                }
                false 
            },
            WN=>{
                let dx = x as i32-self.selected_x as i32;
                let dy = y as i32-self.selected_y as i32;
                if self.board.is_white_piece(x, y){
                    return false; 
                }
                else if [(2,1),(-2,1),(2,-1),(-2,-1),
                (1,2),(-1,2),(1,-2),(-1,-2)].contains(&(dx,dy)){
                    return true;
                }
                false
            }
            BN=>{
                let dx = x as i32-self.selected_x as i32;
                let dy = y as i32-self.selected_y as i32;
                if self.board.is_black_piece(x, y){
                    return false; 
                }
                else if [(2,1),(-2,1),(2,-1),(-2,-1),
                (1,2),(-1,2),(1,-2),(-1,-2)].contains(&(dx,dy)){
                    return true;
                }
                false
            },
            WK=>{
                let dx = x as i32 - self.selected_x as i32;
                let dy = y as i32 - self.selected_y as i32;
                if self.board.is_white_piece(x, y){
                    return false; 
                }
                else if [(1,1),(-1,1),(1,-1),(-1,-1),
                (1,0),(-1,0),(0,-1),(0,1)].contains(&(dx,dy)){
                    return true;
                }
                false
            }
            BK=>{
                let dx = x as i32 - self.selected_x as i32;
                let dy = y as i32 - self.selected_y as i32;
                if self.board.is_black_piece(x, y){
                    return false; 
                }
                else if [(1,1),(-1,1),(1,-1),(-1,-1),
                (1,0),(-1,0),(0,-1),(0,1)].contains(&(dx,dy)){
                    return true;
                }
                false
            }
            _ =>false
        }
    }
    fn run(&mut self){
        self.board.draw();
        let (x,y)=detect_mouse();
        if x!=usize::MAX && y!=usize::MAX && x<self.board.width && y<self.board.height{

            if !self.selected{
                self.selected_x = x;
                self.selected_y = y;

                match self.board.pieces[y][x]{
                    WK | WQ | WR | WN | WB | W =>{
                        self.selected=match self.turn{
                            Turn::White=>true,
                            Turn::Black=>false
                        };
                    },
                    BK | BQ | BR | BN | BB | B=> {
                        self.selected=match self.turn{
                            Turn::White=>false,
                            Turn::Black=>true
                        };
                    },
                    E=>{}
                }
            }else{
                if x==self.selected_x && y == self.selected_y{

                }else if self.is_reachable(x, y){
                    self.board.pieces[y][x] = self.board.pieces[self.selected_y][self.selected_x];
                    self.board.pieces[self.selected_y][self.selected_x]=E;
                    self.turn = match self.turn{
                        Turn::White=>Turn::Black,
                        Turn::Black=>Turn::White
                    }
                }
                self.selected=false;
            }
        }
    
        if self.selected{
            draw_rectangle(self.selected_x as f32*BLOCKSIZE - BUFF, self.selected_y as f32*BLOCKSIZE - BUFF, BLOCKSIZE+2.0*BUFF, BLOCKSIZE+2.0*BUFF, Color::new(0.0,0.0,0.0,0.2));
        }
    }
}

#[macroquad::main("Chess")]
async fn main() {
         let sprite_sheet = load_texture("./../assets/Chess_Pieces_Sprite.png").await.unwrap();
        let mut game=Game{
            turn:Turn::White,
            board:Board::new(sprite_sheet),
            selected:false,
            selected_x:0,
            selected_y:0
        };
    loop {
        clear_background(BLACK);
        game.run();
        next_frame().await
    }
}
