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
