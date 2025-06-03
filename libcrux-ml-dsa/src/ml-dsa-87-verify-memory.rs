use libcrux_ml_dsa::ml_dsa_87;
use libcrux_ml_dsa::types::{MLDSAVerificationKey, MLDSASignature};
use std::fs::read;

fn main() {
    let pk_bytes = read("/tmp/ml-dsa-87-pub-key").expect("");
    let sig_bytes = read("/tmp/ml-dsa-87-sig").expect("");

    let pk = MLDSAVerificationKey::new(pk_bytes.try_into().expect(""));
    let sig = MLDSASignature::new(sig_bytes.try_into().expect(""));

    let message = b"";
    let context = b"";

    let result = ml_dsa_87::verify(&pk, message, context, &sig);

    println!("{}", result.is_ok());
}
