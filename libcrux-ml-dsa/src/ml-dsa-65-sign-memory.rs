use libcrux_ml_dsa::ml_dsa_65;
use rand::rngs::OsRng;
use rand::RngCore;

#[inline(never)]
fn main() {
    let mut randomness = [0u8; 32];
    let mut salt = [0u8; 32];
    Osrng.try_fill_bytes(&mut randomness);
    Osrng.try_fill_bytes(&mut salt);

    let keypair = ml_dsa_65::generate_key_pair(randomness);
    let sk = keypair.signing_key;
    let _pk = keypair.verification_key;

    let message = b"";

    ml_dsa_65::sign(&sk, message, &randomness, salt).expect("");
}
