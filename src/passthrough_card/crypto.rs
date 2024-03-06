use base64;
use base64::Engine;
use base64::engine::general_purpose;
use openssl::rsa::{Padding, Rsa};
use rand::Rng;
use serde::Serialize;
use serde_json;


#[derive(Serialize)]
struct PinBlock {
    pin: String,
    nonce: u128,
}

pub fn encrypt_pin(pin: &str) -> String {
    let mut rng = rand::thread_rng();

    let base: u128 = 10;
    let low: u128 = base.pow(8);
    let high: u128 = base.pow(12);
    let nonce = rng.gen_range(low..high);

    let pin = "1234";

    let pin_block = PinBlock {
        pin: pin.to_string(),
        nonce: nonce,
    };

    let data = serde_json::to_string(&pin_block).unwrap();

    // Encrypt with public key
    // TODO: don't hardcode this path lol
    let public_key_pem = include_bytes!("../../api.lithic.com.pub.pem");

    let rsa = Rsa::public_key_from_pem_pkcs1(public_key_pem).expect("Error prasing public key");
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1_OAEP)
        .unwrap();

    let encrypted_pin_block = general_purpose::STANDARD_NO_PAD.encode(&buf);//base64::encode(&buf);
    encrypted_pin_block
}
