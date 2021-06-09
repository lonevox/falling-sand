use crate::{ParticleSpecies, State};
use palette::LinSrgba;
use lazy_static::lazy_static;

lazy_static! {
    static ref SAND: ParticleSpecies = ParticleSpecies {
        name: "Sand",
        color: LinSrgba::new(1., 1., 1., 1.),
        state: State::Granular,
        mass: 1000,
        combustion_point: std::i32::MAX,
        melting_point: 1700,
        conductivity: 0_f32,
        permeability: 0.5_f32,
        malleability: 0_f32,
        update: |cell, api| {
            println!("sand lol");
        }
    };

    static ref GOLD: ParticleSpecies = ParticleSpecies {
        name: "Gold",
        color: LinSrgba::new(1., 1., 1., 1.),
        state: State::Solid,
        mass: 5000,
        combustion_point: std::i32::MAX,
        melting_point: 1064,
        conductivity: 0.75_f32,
        permeability: 0_f32,
        malleability: 0.8_f32,  // Keep in mind gold is the most malleable metal. I'm leaving some room for exotic materials.
        update: |cell, api| {
            println!("gold lol");
        }
    };

    pub static ref PARTICLE_SPECIES: [&'static ParticleSpecies;2] = [
        &SAND,
        &GOLD,
    ];
}
