mod edges;

use ::rand::Rng;
use macroquad::prelude::*;
use edges::edges;

#[macroquad::main("Simulation")]
async fn main() {
    const RADIUS_PARTICLE: f32 = 20.0;

    struct Particle{
        position: Vec2,
        velocity: Vec2,
    }

    //make the array of dynamic size
    let mut particles = Vec::new();

    //function that returns a Particle type
    fn create_particle() -> Particle{

        let max_number_x = screen_width()-RADIUS_PARTICLE;
        let max_number_y = screen_height()-RADIUS_PARTICLE;

        let mut random_number = ::rand::thread_rng();

        Particle{
            position: Vec2{
                x: random_number.gen_range(RADIUS_PARTICLE..max_number_x),
                y: random_number.gen_range(RADIUS_PARTICLE..max_number_y),
            },
            velocity: Vec2{
                x: random_number.gen_range(-100.00..100.0),
                y: random_number.gen_range(-100.00..100.0),
            },
        }
    }

   //populate the array with the &particles
    for _ in 0..100{
        particles.push(create_particle());
    }
    loop {

        let dt = get_frame_time();
        
        //loop over the array, it's mutable to allow change.
        for particle in &mut particles{
            particle.position += dt * particle.velocity;
            edges(
                particle.position,
                &mut particle.velocity,
                RADIUS_PARTICLE,
                )
        }
        clear_background(BLACK);
        //draw particle element from particles reference (the array)
        for particle in &particles{
            draw_circle(particle.position.x, particle.position.y, RADIUS_PARTICLE, WHITE);
        }

        next_frame().await;
    }
}
