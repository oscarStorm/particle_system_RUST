use macroquad::prelude::*;
const FORCE: f32 = 0.92;
pub fn edges(position: &mut Vec2, velocity: &mut Vec2, radius: f32) {
    if position.x <= radius {
        position.x = radius;
        velocity.x = (velocity.x.abs()) * FORCE;
    } else if position.x >= screen_width() - radius {
        position.x = screen_width() - radius;
        velocity.x = -velocity.x.abs() * FORCE;
    }

    if position.y <= radius {
        position.y = radius;
        velocity.y = velocity.y.abs() * FORCE;
    } else if position.y >= screen_height() - radius {
        position.y = screen_height() - radius;
        velocity.y = -velocity.y.abs() * FORCE;
    }
}

//implement particle overlap and mechanics

//pub fn detect_particle(position: &mut Vec2, velocity: &mut Vec2, radius: f32){
//   if position.x >
//}
