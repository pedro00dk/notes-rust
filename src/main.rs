mod nbodyv1;

fn main() {
    unsafe {
        nbodyv1::offset_momentun(nbodyv1::solarBodies.as_mut_ptr());
        nbodyv1::output_energy(nbodyv1::solarBodies.as_mut_ptr());
        let c = std::env::args().nth(1).unwrap()  // Note 2
            .parse().unwrap();
        for _ in 0..c {
            nbodyv1::advance(nbodyv1::solarBodies.as_mut_ptr())
        }
        nbodyv1::output_energy(nbodyv1::solarBodies.as_mut_ptr());
    }
}
