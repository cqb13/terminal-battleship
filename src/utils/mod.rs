pub mod ships;
pub mod terminal;

use rand::Rng;

pub fn random_50_50() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}
