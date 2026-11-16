use totp_rs::{Algorithm, Builder, Secret, Totp};

use std::error::Error;

const KEYRING_SERVICE: &str = "xiv-otp";
const KEYRING_USER: &str = "totp-secret";

pub fn setup() -> Result<(), Box<dyn Error>> {
    let secret = rpassword::prompt_password("Base32 TOTP secret (hidden): ")?;
    entry()?.set_password(&secret)?;
    println!("Saved. Restart xiv-otp to use it.");
    Ok(())
}

pub fn load() -> Result<Totp, Box<dyn Error>> {
    let secret = entry()?
        .get_password()
        .map_err(|e| format!("{e}; run `xiv-otp --setup` first"))?;
    make_otp(&secret)
}

pub fn entry() -> keyring::Result<keyring::Entry> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
}

pub fn make_otp(secret: &str) -> Result<Totp, Box<dyn Error>> {
    let key = Secret::try_from_base32(secret.replace([' ', '-', '='], "").to_uppercase())
        .map_err(|_| "invalid base32 secret")?;

    if key.is_empty() {
        return Err("empty secret".into());
    }

    Ok(Builder::new()
        .with_algorithm(Algorithm::SHA1)
        .with_digits(6)
        .with_skew(1)
        .with_step_duration(30)
        .with_secret(key)
        .build_noncompliant())
}

#[test]
fn make_totp_rfc6238() {
    // RFC 6238 SHA-1 test vector (94287082 at t=59, last 6 digits), spaced, dashed, lowercase and padded as users paste it.
    let totp = make_otp("gezd gnbv-gy3t qojq gezd gnbv gy3t qojq==").unwrap();
    assert_eq!(totp.generate(59).to_string(), "287082");
    assert!(make_otp("").is_err());
}
