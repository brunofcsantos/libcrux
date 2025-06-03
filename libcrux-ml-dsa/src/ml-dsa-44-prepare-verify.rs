use libcrux_ml_dsa::ml_dsa_44;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut keygen_randomness = [0u8; 32];
    let mut signing_randomness = [0u8; 32];
    OsRng.fill_bytes(&mut keygen_randomness);
    OsRng.fill_bytes(&mut signing_randomness);

    let context = b"";

    let keypair = ml_dsa_44::generate_key_pair(keygen_randomness);
    let sk = keypair.signing_key;
    let pk = keypair.verification_key;

    let message = b"";
    let sig = ml_dsa_44::sign(&sk, message, context, signing_randomness)
        .map_err(|e| format!("signing failed: {:?}", e))?;

    write("/tmp/ml-dsa-44-pub-key", pk.as_slice())?;
    write("/tmp/ml-dsa-44-sig", sig.as_slice())?;

    Ok(())
}
