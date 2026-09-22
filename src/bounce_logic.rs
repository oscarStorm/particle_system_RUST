use crate::particle_behaviour::Particle;
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

//implemen particle overlap and mechanic

pub fn detect_particle(particles: &mut [Particle]) {
    //sets 2, the first loop finds the first element,
    //the second find the second and compares
    //its then back to the first ++ after the second loop is done.
    for i in 0..particles.len() {
        //splits the array into slices to prove the element is not accesed twice
        let (a_and_previous, after_a) = particles.split_at_mut(i + 1);
        //sets a = to the element of the sliced array of index i
        let a = &mut a_and_previous[i];

        for b in after_a.iter_mut() {
            // `offset` points from a's centre toward b's centre.
            let offset = b.position - a.position;
            let distance = offset.length();
            let touching_distance = a.radius + b.radius;

            if distance < touching_distance {
                // Move the particles apart so their edges are exactly touching.
                // Splitting the correction equally makes neither particle "win".
                let direction = if distance > 0.0 {
                    offset / distance
                } else {
                    // Two centres can occasionally be exactly equal; in that case
                    // choose an arbitrary direction so division by zero is avoided.
                    vec2(1.0, 0.0)
                };
                let overlap = touching_distance - distance;

                a.position -= direction * (overlap / 2.0);
                b.position += direction * (overlap / 2.0);

                // Only bounce if they are moving toward each other. Otherwise,
                // particles that have just been separated would bounce again.
                let relative_velocity = b.velocity - a.velocity;
                if relative_velocity.dot(direction) < 0.0 {
                    a.velocity = -a.velocity;
                    b.velocity = -b.velocity;
                }
            }
        }
    }
}
//
//
//
//
//
//
//
//
//
//
