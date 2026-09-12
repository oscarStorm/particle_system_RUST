use macroquad::prelude::*;
const FORCE: f32 = 1.0;
pub fn edges(position: Vec2, velocity: &mut Vec2, radius: f32){

    if position.x <= radius {
        velocity.x = (velocity.x.abs())*FORCE;
    } else if position.x >= screen_width() - radius {
        velocity.x = -velocity.x.abs()*FORCE;
    }
    
    if position.y <= radius {
        velocity.y = velocity.y.abs()*FORCE;
    } else if position.y >= screen_height() - radius {
        velocity.y = -velocity.y.abs()*FORCE;
    }
}
