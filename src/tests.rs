use crate::World;
use crate::grid_util::*;

/// A set of ParticleSpecies to use within tests.
mod game_of_life {
    use crate::ParticleSpecies;
    use palette::LinSrgba;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref ALIVE: ParticleSpecies = ParticleSpecies::new_default_physical(
            "Alive",
            LinSrgba::new(1., 1., 1., 1.),
            |cell, api| {

            }
        );

        static ref DEAD: ParticleSpecies = ParticleSpecies::new_default_physical(
            "Dead",
            LinSrgba::new(0., 0., 0., 1.),
            |cell, api| {

            }
        );

        pub static ref PARTICLE_SPECIES: [&'static ParticleSpecies;2] = [
            &ALIVE,
            &DEAD,
        ];
    }
}

#[test]
fn test_new_empty_world() {
    World::new_empty(
        Size::new(4, 4),
        Size::new(16, 16),
        &*game_of_life::PARTICLE_SPECIES,
    );
}