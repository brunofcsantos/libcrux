use libcrux_ml_dsa::ml_dsa_87;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut keygen_randomness = [0u8; 32];
    let mut signing_randomness = [0u8; 32];
    Osrng.try_fill_bytes(&mut keygen_randomness);
    Osrng.try_fill_bytes(&mut signing_randomness);

    let context = b"";

    let keypair = ml_dsa_87::generate_key_pair(keygen_randomness);
    let sk = keypair.signing_key;
    let pk = keypair.verification_key;

    let message = b"";
    let sig = ml_dsa_87::sign(&sk, message, context, signing_randomness)
        .map_err(|e| format!("signing failed: {:?}", e))?;

    write("/tmp/ml-dsa-87-pub-key", pk.as_slice())?;
    write("/tmp/ml-dsa-87-sig", sig.as_slice())?;

    Ok(())
}
