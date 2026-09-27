mod bounce_logic;
mod particle_behaviour;
use bounce_logic::{detect_between, detect_particle};
use macroquad::prelude::*;
use particle_behaviour::create_particles;
use particle_behaviour::update_particle;
use particle_behaviour::{Particle, shoot_particle, shoot_particle_right};

#[macroquad::main("Simulation")]
async fn main() {
    //takes ownership of the array particles
    let mut particles = create_particles();
    let mut shooting_particles: Vec<Particle> = Vec::new();

    //loop for frame and rendering
    loop {
        //delta for framerate
        let dt = get_frame_time();
        clear_background(BLACK);
        if is_key_pressed(KeyCode::R) {
            particles = create_particles();
        }
        if is_key_pressed(KeyCode::S) {
            shooting_particles.push(shoot_particle());
        }

        if is_key_pressed(KeyCode::K) {
            shooting_particles.push(shoot_particle_right());
        }

        // Move both groups first. Only the original particles bounce at edges.
        update_particle(&mut particles, dt, true);
        update_particle(&mut shooting_particles, dt, false);

        // Check pairs within each group, then pairs across the two groups.
        // All three checks reuse the same collision calculation.
        detect_particle(&mut particles);
        detect_particle(&mut shooting_particles);
        detect_between(&mut particles, &mut shooting_particles);

        //draw particle element from particles reference (the array)
        for particle in &particles {
            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.radius,
                BLUE,
            );
        }
        for particle in &shooting_particles {
            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.radius,
                RED,
            );
        }
        next_frame().await;
    }
}
