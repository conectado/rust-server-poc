use base64::{decode_config, encode_config, STANDARD_NO_PAD};
use ring::{digest, error::Unspecified, pbkdf2};
use std::num::NonZeroU32;
// TODO: Constants should be loaded ffrom a config file
// This will make slight tuning to the security easier
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

/// Generate password from string
///
/// Generate a password from string for a given config.
/// the config should be loaded from a config file not
/// documented yet.
///
/// * `password` - The password to be hashed
/// ---
/// Returns a String with a b64 encoded hash
pub fn generate_password_hash(password: &str) -> String {
    let mut result: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(1000).unwrap(),
        "TEMPORAL SECRET".as_bytes(),
        password.as_bytes(),
        &mut result,
    );

    // Gosh, We need an slice over result since AsRef
    // isn't implemented for arrays longer than 32 bits
    // I know this is a shitty limitation, I expect it to be gone
    // soon, when rust can declare traits on arrays of arbitrary size
    // Meanwhile, this work-around has little impact(Still this sucks :( )
    encode_config(&result[..], STANDARD_NO_PAD)
}

/// Verify password, given a previously generated password
///
/// Verifies a password, given a previously generated password
/// The previously generated password should be passed exactly as generated
/// the testing password should be passed as plain string
///
/// * `original_password` - the previously generated password by
/// `crate::password_manager::generate_password_hash`
/// * `testing_password` - plain text password
/// ---
/// Returns either Ok() or a `Ring::Unspecified` error
pub fn verify_password(original_password: &str, testing_password: &str) -> Result<(), Unspecified> {
    let decoded_original_password = decode_config(original_password, STANDARD_NO_PAD).unwrap();
    pbkdf2::verify(
        PBKDF2_ALG,
        NonZeroU32::new(1000).unwrap(),
        "TEMPORAL SECRET".as_bytes(),
        testing_password.as_bytes(),
        &decoded_original_password[..],
    )
}
