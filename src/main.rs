mod grid_util;
mod particle_species_sets;

use crate::grid_util::*;
use rand::random;
use palette::LinSrgba;

enum State {
    Solid,
    Liquid,
    Gas,
    Granular,
}

pub struct ParticleSpecies {
    name: &'static str,
    color: LinSrgba,
    state: State,
    mass: i32,                      // Grams
    combustion_point: i32,          // Degrees celsius
    melting_point: i32,             // Degrees celsius
    conductivity: f32,              // 0 to 1, 0 meaning that no energy can travel through the Particle, 1 meaning energy can travel through the Particle at the speed of light and no energy is transfered to heat when passing through the Particle.
    permeability: f32,              // 0 to 1, 1 meaning liquids and gasses can fully pass through the Particle without any resistance.
    malleability: f32,              // 0 to 1, 1 meaning any force on the Particle will deform it without any resistance.
    pub update: fn(particle: &Cell, api: ParticleApi),
}

impl ParticleSpecies {
    /// Creates a ParticleSpecies with default values for all physical properties.
    /// Useful for when it doesn't make sense for a species to have defined physical properties.
    pub fn new_default_physical(name: &'static str, color: LinSrgba, update: fn(particle: &Cell, api: ParticleApi)) -> Self {
        Self {
            name,
            color,
            state: State::Solid,
            mass: 1000,
            combustion_point: std::i32::MAX,
            melting_point: std::i32::MAX,
            conductivity: 0_f32,
            permeability: 0_f32,
            malleability: 0_f32,
            update,
        }
    }

    //fn new(name: &'static str) -> Self {Self{name}}
    pub fn brittle(&self) -> bool { return self.malleability < 0.1; }
}

pub struct Cell<'a> {
    species: &'a ParticleSpecies,
    /// 8-bit register that initially stores a random value.
    ra: u8,
    /// 8-bit register.
    rb: u8,
    /// True when this Particle has been processed in the current update call.
    processed: bool,
    brightness: u8,
}

impl<'a> Cell<'a> {
    pub fn new(species: &'a ParticleSpecies) -> Self {
        Self {
            species,
            ra: random(),
            rb: 0,
            processed: false,
            brightness: 1,
        }
    }

    pub fn update(&self, api: ParticleApi) {
        (self.species.update)(self, api);
    }
}

struct Chunk<'a> {
    /// The size of the chunks in particles.
    size: Size,
    /// Chunk position within the world (in chunks, not cells).
    position: Position,
    /// The bounding box encompassing the cells of the chunk. This is in world space.
    bounds: Rectangle,
    cells: Vec<Cell<'a>>,
}

impl<'a> Chunk<'a> {
    fn new(size: Size, position: Position) -> Self {
        Self {
            size,
            position,
            bounds: Rectangle::new(
                position * Position::from(size),
                size
            ),
            cells: Vec::new()
        }
    }
    
    fn update(&self) {
        /*for y in 0..self.world.chunk_height {
            for x in 0..self.world.chunk_width {
                let particle = self.get_particle(x, y);
            }
        }*/
        for (i, particle) in self.cells.iter().enumerate() {
            if particle.processed { return };
            particle.update(
                ParticleApi {
                    chunk: self,
                    position: Position::new(i as i32 % self.size.width as i32, i as i32 / self.size.height as i32),
                }
            );
        }
    }
}

struct World<'a> {
    /// The size of the world in chunks.
    size: Size,
    /// The species of particle that can exist within the world.
    particle_species: &'static [&'static ParticleSpecies],
    chunks: Vec<Chunk<'a>>,
}

impl<'a> World<'a> {
    fn new_empty(size: Size, chunk_size: Size, particle_species: &'static [&'static ParticleSpecies]) -> Self {
        // Create chunks.
        let mut chunks: Vec<Chunk<'a>> = Vec::new();
        for i in 0..(size.width * size.height) {
            let chunk: Chunk<'a> = Chunk::new(chunk_size, Position::from(size));
            chunks.push(chunk);
        }

        Self {size, particle_species, chunks}
    }
    
    fn generate(generator: &ParticleGenerator) {
        //generator.
        //World {}
    }
    
    fn update(&self) {
        for chunk in &self.chunks {
            chunk.update();
        }
    }

    fn get_chunk(&self, x: u32, y: u32) -> Option<&Chunk> {
        return self.chunks.get((x * y) as usize);
    }
}

struct ParticleGenerator {
    
}

pub struct ParticleApi<'a> {
    chunk: &'a Chunk<'a>,
    position: Position,
}

impl<'a> ParticleApi<'a> {
    /// Gets a cell in the neighbourhood using relative coordinates.
    fn get_cell(&self, relative_position: Position) -> Option<&Cell> {
        if !self.valid_relative_position(relative_position) { return None; }
        return None;
    }

    /// Sets a cell in the neighbourhood using relative coordinates.
    fn set_cell() {

    }

    /// Ensures the absolute position of the given relative position is within the chunk.
    fn valid_relative_position(&self, relative_position: Position) -> bool {
        let absolute_position = self.position + relative_position;
        self.chunk.bounds.contains_position(absolute_position)
    }
}

fn main() {
    let world = World::new_empty(
        Size::new(10, 10),
        Size::new(64, 64),
        &*particle_species_sets::basic::PARTICLE_SPECIES,
    );
    world.update();
}
