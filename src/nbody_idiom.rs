use std::f64::consts::PI;

#[derive(Clone, Debug)]
struct Body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

const BODIES_COUNT: usize = 5;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const INTERACTIONS_COUNT: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;

const INITIAL_STATE: [Body; BODIES_COUNT] = [
    // Sun
    Body {
        mass: SOLAR_MASS,
        position: [0.; 3],
        velocity: [0.; 3],
    },
    // Jupiter
    Body {
        position: [
            4.841_431_442_464_72e0,
            -1.160_320_044_027_428_4e0,
            -1.036_220_444_711_231_1e-1,
        ],
        velocity: [
            1.660_076_642_744_037e-3 * DAYS_PER_YEAR,
            7.699_011_184_197_404e-3 * DAYS_PER_YEAR,
            -6.904_600_169_720_63e-5 * DAYS_PER_YEAR,
        ],
        mass: 9.547_919_384_243_266e-4 * SOLAR_MASS,
    },
    // Saturn
    Body {
        position: [
            8.343_366_718_244_58e0,
            4.124_798_564_124_305e0,
            -4.035_234_171_143_214e-1,
        ],
        velocity: [
            -2.767_425_107_268_624e-3 * DAYS_PER_YEAR,
            4.998_528_012_349_172e-3 * DAYS_PER_YEAR,
            2.304_172_975_737_639_3e-5 * DAYS_PER_YEAR,
        ],
        mass: 2.858_859_806_661_308e-4 * SOLAR_MASS,
    },
    // Uranus
    Body {
        position: [
            1.289_436_956_213_913_1e1,
            -1.511_115_140_169_863_1e1,
            -2.233_075_788_926_557_3e-1,
        ],
        velocity: [
            2.964_601_375_647_616e-3 * DAYS_PER_YEAR,
            2.378_471_739_594_809_5e-3 * DAYS_PER_YEAR,
            -2.965_895_685_402_375_6e-5 * DAYS_PER_YEAR,
        ],
        mass: 4.366_244_043_351_563e-5 * SOLAR_MASS,
    },
    // Neptune
    Body {
        position: [
            1.537_969_711_485_091_1e1,
            -2.591_931_460_998_796_4e1,
            1.792_587_729_503_711_8e-1,
        ],
        velocity: [
            2.680_677_724_903_893_2e-3 * DAYS_PER_YEAR,
            1.628_241_700_382_423e-3 * DAYS_PER_YEAR,
            -9.515_922_545_197_159e-5 * DAYS_PER_YEAR,
        ],
        mass: 5.151_389_020_466_114_5e-5 * SOLAR_MASS,
    },
];

fn offset_momentum(bodies: &mut [Body; BODIES_COUNT]) {
    let (sun, planets) = bodies.split_first_mut().unwrap();
    sun.velocity = [0.0; 3];
    for planet in planets {
        for m in 0..3 {
            sun.velocity[m] -= planet.velocity[m] * planet.mass / SOLAR_MASS;
        }
    }
}

fn output_energy(bodies: &mut [Body; BODIES_COUNT]) {
    let mut energy = 0.0;
    for (i, body) in bodies.iter().enumerate() {
        energy += 0.5
            * body.mass
            * (body.velocity[0].powi(2) + body.velocity[1].powi(2) + body.velocity[2].powi(2));

        for body2 in &bodies[i + 1..BODIES_COUNT] {
            energy -= body.mass * body2.mass
                / ((body.position[0] - body2.position[0]).powi(2)
                    + (body.position[1] - body2.position[1]).powi(2)
                    + (body.position[2] - body2.position[2]).powi(2))
                .sqrt();
        }
    }
    println!("{:.9}", energy);
}

fn advance(bodies: &mut [Body; BODIES_COUNT]) {
    let mut position_deltas = [[0.; 3]; INTERACTIONS_COUNT];
    {
        let mut k = 0;
        for i in 0..BODIES_COUNT - 1 {
            for j in i + 1..BODIES_COUNT {
                for (m, pd) in position_deltas[k].iter_mut().enumerate() {
                    *pd = bodies[i].position[m] - bodies[j].position[m];
                }
                k += 1;
            }
        }
    }

    let magnitudes = {
        let mut magnitudes = [0.0; INTERACTIONS_COUNT];
        for (i, mag) in magnitudes.iter_mut().enumerate() {
            let distance_squared = position_deltas[i][0].powi(2)
                + position_deltas[i][1].powi(2)
                + position_deltas[i][2].powi(2);

            *mag = 0.01 / (distance_squared * distance_squared.sqrt());
        }
        magnitudes
    };

    {
        let mut k = 0;
        for i in 0..BODIES_COUNT - 1 {
            for j in i + 1..BODIES_COUNT {
                let i_mass_mag = bodies[i].mass * magnitudes[k];
                let j_mass_mag = bodies[j].mass * magnitudes[k];
                for (m, pd) in position_deltas[k].iter().enumerate() {
                    bodies[i].velocity[m] -= *pd * j_mass_mag;
                    bodies[j].velocity[m] += *pd * i_mass_mag;
                }
                k += 1;
            }
        }
    }

    for body in bodies {
        for (m, pos) in body.position.iter_mut().enumerate() {
            *pos += 0.01 * body.velocity[m];
        }
    }
}

pub fn main() {
    let mut solar_bodies = INITIAL_STATE;
    offset_momentum(&mut solar_bodies);
    output_energy(&mut solar_bodies);
    let c = std::env::args().nth(1).unwrap().parse().unwrap();
    for _ in 0..c {
        advance(&mut solar_bodies);
    }
    output_energy(&mut solar_bodies);
}
