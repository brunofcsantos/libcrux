use libcrux_ml_dsa::ml_dsa_65;
use rand::rngs::OsRng;
use rand::RngCore;

// This is a simple script for measuring the memory use of ml_dsa_65::generate_key_pair

#[inline(never)]
fn main() {
    let mut randomness = [0u8; 32];
    OsRng.fill_bytes(&mut randomness);

    ml_dsa_65::generate_key_pair(randomness);

}
