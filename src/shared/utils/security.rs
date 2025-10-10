use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::distributions::{Distribution, Uniform};

use crate::shared::errors::Rfc7807Problem;

const SPECIALS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:',.<>?/";
const MIN_LENGTH: usize = 8;
const WEAK_PASSWORD_MESSAGE: &str =
    "La contraseña debe tener al menos 8 caracteres, mayúsculas, minúsculas, caracteres especiales y números";

/// Hashes a password using Argon2i with the configured salt rounds
pub fn hash_password(password: &str) -> Result<String, Box<Rfc7807Problem>> {
    let salt = SaltString::generate(&mut OsRng);

    // Use Argon2 with default configuration (equivalent to Config::original())
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            Box::new(Rfc7807Problem::server_error(
                Some("Error al procesar contraseña"),
                Some("No se pudo generar el hash de la contraseña"),
                Some("PASSWORD_HASH_ERROR"),
            ))
        })?
        .to_string();

    Ok(password_hash)
}

/// Verifies a password against a hashed password using Argon2
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, Box<Rfc7807Problem>> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        tracing::error!("Failed to parse password hash: {}", e);
        Box::new(Rfc7807Problem::server_error(
            Some("Error al verificar contraseña"),
            Some("El hash almacenado es inválido"),
            Some("INVALID_PASSWORD_HASH"),
        ))
    })?;

    // Verify using any Argon2 variant (detects from hash)
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => {
            tracing::error!("Failed to verify password: {}", e);
            Err(Box::new(Rfc7807Problem::server_error(
                Some("Error al verificar contraseña"),
                Some("Error en el proceso de verificación"),
                Some("PASSWORD_VERIFY_ERROR"),
            )))
        }
    }
}

/// O(n) single-pass ASCII validator with early-exit.
/// Uses ASCII class detection for speed. Non-ASCII chars do not satisfy any class.
pub fn validate_secure_password(password: &str) -> Option<&'static str> {
    if password.len() < MIN_LENGTH {
        return Some(WEAK_PASSWORD_MESSAGE);
    }

    let mut flags: u8 = 0;
    const UPPER: u8 = 1 << 0;
    const LOWER: u8 = 1 << 1;
    const DIGIT: u8 = 1 << 2;
    const SPEC: u8 = 1 << 3;
    const ALL: u8 = UPPER | LOWER | DIGIT | SPEC;

    // Iterate bytes (faster than Unicode scalar iteration for these rules)
    for &b in password.as_bytes() {
        match b {
            b'A'..=b'Z' => flags |= UPPER,
            b'a'..=b'z' => flags |= LOWER,
            b'0'..=b'9' => flags |= DIGIT,
            _ => {
                if SPECIALS.contains(&b) {
                    flags |= SPEC
                }
            }
        }
        if flags == ALL {
            return None;
        } // early exit
    }

    Some(WEAK_PASSWORD_MESSAGE)
}

#[inline]
fn rand_pick<'a>(rng: &mut impl rand::Rng, from: &'a [u8]) -> &'a u8 {
    let dist = Uniform::new(0usize, from.len());
    &from[dist.sample(rng)]
}

/// CSPRNG (OsRng) generator. Guarantees all classes present. Fisher–Yates shuffle.
/// No temporary allocations; ASCII only => always valid UTF-8.
pub fn generate_secure_password(min_length: usize) -> String {
    const UPPERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";

    let mut rng = rand::thread_rng();

    let base = std::cmp::max(min_length, MIN_LENGTH);
    let jitter = Uniform::new_inclusive(0usize, 4usize).sample(&mut rng);
    let len = base + jitter;

    let mut out = Vec::with_capacity(len);
    out.push(*rand_pick(&mut rng, UPPERS));
    out.push(*rand_pick(&mut rng, LOWERS));
    out.push(*rand_pick(&mut rng, DIGITS));
    out.push(*rand_pick(&mut rng, SPECIALS));

    let class_pick = Uniform::new(0u8, 4u8);
    while out.len() < len {
        match class_pick.sample(&mut rng) {
            0 => out.push(*rand_pick(&mut rng, UPPERS)),
            1 => out.push(*rand_pick(&mut rng, LOWERS)),
            2 => out.push(*rand_pick(&mut rng, DIGITS)),
            _ => out.push(*rand_pick(&mut rng, SPECIALS)),
        }
    }

    // Fisher-Yates shuffle
    for i in (1..out.len()).rev() {
        let j = Uniform::new_inclusive(0usize, i).sample(&mut rng);
        out.swap(i, j);
    }

    String::from_utf8(out).expect("ASCII is valid UTF-8")
}
