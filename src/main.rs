mod bounce_logic;
mod particle_behaviour;
use macroquad::prelude::*;
use particle_behaviour::create_particles;
use particle_behaviour::update_particle;

#[macroquad::main("Simulation")]
async fn main() {
    //takes ownership of the array particles
    let mut particles = create_particles();

    //loop for frame and rendering
    loop {
        //delta for framerate
        let dt = get_frame_time();
        clear_background(BLACK);
        update_particle(&mut particles, dt);

        //draw particle element from particles reference (the array)
        for particle in &particles {
            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.radius,
                BLUE,
            );
        }
        next_frame().await;
    }
}
