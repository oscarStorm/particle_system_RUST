Particle System — Rust
An interactive 2D particle simulation written in Rust using Macroquad.

I built this project to improve my understanding of Rust and experiment with vector mathematics, collision detection, and real-time simulation. The physics and collision-handling logic are implemented in Rust; Macroquad handles window creation, input, and rendering.
Features
- Randomized particles: Starts with 20 particles of varying sizes, masses, positions, and velocities.
- Particle collisions: Detects overlapping circles, separates them, and calculates mass-dependent collision responses.
- Boundary collisions: Particles bounce off window edges with damping and floor friction.
- Interactive projectiles: Fire fast-moving particles from either side of the window and observe their interactions with the simulation.
- Frame-independent movement: Updates particle positions using the elapsed time between frames (dt).
- Separate simulation logic: Particle behavior and collision handling are implemented in separate Rust modules.
Controls
Key	Action
R	Regenerate the original 20 particles
S	Shoot a red particle from the left
K	Shoot a red particle from the right


The original particles are drawn in blue, while fired particles are drawn in red. Pressing R regenerates the blue particles; it does not clear particles already fired.
Getting Started
You need Rust and Cargo installed and a desktop environment that supports graphics.
git clone https://github.com/oscarStorm/particle_system_RUST.git
cd particle_system_RUST
cargo run --release
Cargo downloads and builds the dependencies automatically. You can also run cargo run for a development build.
How It Works
Each particle has a position, velocity, radius, and mass. On every frame, the program:
1. Updates particle positions using position += velocity * dt.
2. Applies boundary collisions to the original group of particles.
3. Checks collisions within each group and between the two groups.
4. Draws the updated particles using Macroquad.
Collision handling
For two overlapping circles, the simulation calculates the direction between their centers, moves them apart, and projects their velocities onto that direction. It then applies a mass-dependent, one-dimensional elastic-collision calculation along the collision normal and dampens the resulting velocity changes.

This is a simplified, discrete collision simulation, rather than a full physics engine. Gravity is present as commented-out code and is not currently enabled.
Rust concepts explored
- Ownership and mutable borrowing with &mut references.
- Using split_at_mut to safely modify two different particles in the same collection.
- Working with Vec<Particle> and mutable slices.
- 2D vector operations, including normalization and dot products.
- Real-time update and rendering loops.
Collision detection checks particle pairs directly, so its cost grows approximately quadratically with the number of particles.
Project Structure
src/
├── main.rs                 # Application loop, input, and rendering
├── particle_behaviour.rs   # Particle creation and movement
└── bounce_logic.rs         # Boundary and particle collision responses
Tech Stack
- Rust (2024 edition)
- Macroquad 0.4.16 — rendering, window management, keyboard input, and 2D vectors
- rand 0.8.5 — randomized initial particle properties
Possible Improvements
- Enable configurable gravity.
- Add controls for the number of particles and simulation parameters.
- Use spatial partitioning to reduce collision checks for larger simulations.
- Improve collision handling for fast-moving particles.
A learning project focused on Rust, simulation mechanics, and the mathematics behind collisions.
