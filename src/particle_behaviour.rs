use ::rand::Rng;
use macroquad::prelude::*;

use crate::bounce_logic::{detect_particle, edges};

const GRAVITY_ACCELERATION: f32 = 500.0;

pub struct Particle {
    pub mass: f32,
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}
//function that returns a Particle type
pub fn particles_initialize() -> Particle {
    let radius: f32 = 5.0;
    let max_number_x = screen_width() - radius;
    let max_number_y = screen_height() - radius;
    let mut random_number = ::rand::thread_rng();

    Particle {
        mass: 10.0,
        radius,
        position: Vec2 {
            x: random_number.gen_range(radius..max_number_x),
            y: random_number.gen_range(radius..max_number_y),
        },
        velocity: Vec2 {
            x: random_number.gen_range(-100.00..100.0),
            y: random_number.gen_range(-100.00..100.0),
        },
    }
}
pub fn create_particles() -> Vec<Particle> {
    let mut particles = Vec::new();

    for _ in 0..500 {
        particles.push(particles_initialize());
    }
    particles
}

//[Particle] refers to a slice type: a sequence of particles
pub fn update_particle(particles: &mut [Particle], dt: f32) {
    //loop over the vector, it's mutable to allow change.
    for particle in particles.iter_mut() {
        //gravity for the particles
        particle.velocity.y += GRAVITY_ACCELERATION * dt;
        //velocity for the particle
        particle.position += dt * particle.velocity;
        //applies edge logic
        edges(
            &mut particle.position,
            &mut particle.velocity,
            particle.radius,
        )
    }
    detect_particle(particles);
}
