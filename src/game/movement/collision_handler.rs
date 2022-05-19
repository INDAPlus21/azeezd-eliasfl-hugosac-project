use crate::game::{BLOCK_SIZE_FROM_CENTER, HEAD_HEIGHT, HEIGHT, PLAYER_SIZE_FROM_CENTER};

pub fn collision(current: [f32; 3], new_pos: [f32; 3], block: [f32; 3]) -> bool {
    // Current position in block, 1=positive side 2=negative side
    let x1 = current[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
    let x2 = current[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;

    let y1 = block[1] + BLOCK_SIZE_FROM_CENTER > current[1] - HEIGHT; // Touch at feet
    let y2 = block[1] - BLOCK_SIZE_FROM_CENTER < current[1] + HEAD_HEIGHT; // Touch at top of head (has a height)

    let z1 = current[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
    let z2 = current[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;

    // New position will touch block 1=positive, 2=negative
    let xn1 = new_pos[0] - PLAYER_SIZE_FROM_CENTER < block[0] + BLOCK_SIZE_FROM_CENTER;
    let xn2 = new_pos[0] + PLAYER_SIZE_FROM_CENTER > block[0] - BLOCK_SIZE_FROM_CENTER;

    let yn1 = block[1] + BLOCK_SIZE_FROM_CENTER > new_pos[1] - HEIGHT; // Touch at feet
    let yn2 = block[1] - BLOCK_SIZE_FROM_CENTER < new_pos[1] + HEAD_HEIGHT; // Touch at top of head (has a height)

    let zn1 = new_pos[2] - PLAYER_SIZE_FROM_CENTER < block[2] + BLOCK_SIZE_FROM_CENTER;
    let zn2 = new_pos[2] + PLAYER_SIZE_FROM_CENTER > block[2] - BLOCK_SIZE_FROM_CENTER;

    return (((xn1 && x2) || (x1 && xn2)) && y1 && y2 && z1 && z2) // Touch happens in x
        || (x1 && x2 && ((yn1 && y2) || (y1 && yn2))&& z1 && z2) // Touch happens in y
        || (x1 && x2 && y1 && y2  && ((zn1 && z2) || (z1 && zn2))) // Touch happens in z
    ;
}
