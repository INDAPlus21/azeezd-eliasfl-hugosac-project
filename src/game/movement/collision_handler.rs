use crate::game::{BLOCK_SIZE_FROM_CENTER, HEIGHT, PLAYER_SIZE_FROM_CENTER};

pub struct CollisionHandler {
    pub x_collision: bool,
    pub y_collision: bool,
    pub z_collision: bool
}

impl CollisionHandler {
    pub fn new(current: [f32; 3], new_pos: [f32; 3], block: [f32; 3]) -> Self {
        
        let x1 = current[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
        let x2 = current[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;
        let y1 = block[1] + BLOCK_SIZE_FROM_CENTER > current[1] - HEIGHT + BLOCK_SIZE_FROM_CENTER;
        let y2 = block[1] - BLOCK_SIZE_FROM_CENTER < current[1];
        let z1 = current[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
        let z2 = current[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;
        let xn1 = new_pos[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
        let xn2 = new_pos[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;
        let yn1 = block[1] + BLOCK_SIZE_FROM_CENTER > new_pos[1] - HEIGHT + BLOCK_SIZE_FROM_CENTER;
        let yn2 = block[1] - BLOCK_SIZE_FROM_CENTER < new_pos[1];
        let zn1 = new_pos[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
        let zn2 = new_pos[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;
        
        Self {
            x_collision: y1 && y2 && ((xn1 && x2) || (x1 && xn2)) && z1 && z2,
            y_collision: x1 && x2 && z1 && z2 && ((yn1 && y2) || (y1 && yn2)),
            z_collision: y1 && y2 && x1 && x2 && ((zn1 && z2) || (z1 && zn2)),
        }
    }
}