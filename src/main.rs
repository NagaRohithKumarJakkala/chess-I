use macroquad::prelude::*;
const BLOCKSIZE:f32 = 75.0;
const SPRITESIZE:f32 = 45.0;

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

    fn detect_mouse(&self){
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x,y)= mouse_position();
            let i:usize = (x/BLOCKSIZE) as usize;
            let j:usize = (y/BLOCKSIZE) as usize;

            if i<self.width && j<self.height{
                draw_rectangle(i as f32*BLOCKSIZE, j as f32*BLOCKSIZE, BLOCKSIZE, BLOCKSIZE, BLACK);
            }
        }
    }
}

#[macroquad::main("Chess")]
async fn main() {
         let sprite_sheet = load_texture("./../assets/Chess_Pieces_Sprite.png").await.unwrap();
        let board = Board::new(sprite_sheet);
    loop {
        clear_background(BLACK);

        board.draw();
        board.detect_mouse();

        next_frame().await
    }
}
