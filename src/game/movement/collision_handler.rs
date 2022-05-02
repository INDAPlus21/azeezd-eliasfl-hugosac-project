use crate::game::{BLOCK_SIZE_FROM_CENTER, HEIGHT, PLAYER_SIZE_FROM_CENTER};

pub struct CollisionHandler {
    pub x_collision: bool,
    pub z_collision: bool
}

impl CollisionHandler {
    pub fn new(current: [f32; 3], new_pos: [f32; 3], block: [f32; 3]) -> Self {
        let in_y = new_pos[1] - HEIGHT > block[1] - 2. * BLOCK_SIZE_FROM_CENTER && new_pos[1] - HEIGHT < block[1];
        
        let x1 = current[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
        let x2 = current[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;
        let z1 = current[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
        let z2 = current[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;
        let xn1 = new_pos[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
        let xn2 = new_pos[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;
        let zn1 = new_pos[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
        let zn2 = new_pos[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;
        
        Self {
            x_collision: in_y && ((xn1 && x2) || (x1 && xn2)) && z1 && z2,
            z_collision: in_y && x1 && x2 && ((zn1 && z2) || (z1 && zn2)),
        }
    }
}