use std::fs;

/// This variable is the length of the salts used for hashing over the password
pub const HASH_SALTS: usize = 10;

#[derive(Debug)]
pub struct SecretsVariables {
    pub jwt_password: String,
    pub jwt_at_exp: String,
    pub jwt_rt_exp: String,
    pub dsn: String,
    pub root_email: String,
    pub root_password: String,
    pub root_dni: String,
    pub root_social_reason: String,
    pub environment: String,
    pub jwt_aud: String,
    pub jwt_kid: String,
    pub support_email: String,
}

impl Default for SecretsVariables {
    fn default() -> Self {
        SecretsVariables {
            jwt_password: Self::value("jwt_password"),
            jwt_at_exp: Self::value("jwt_at_exp"),
            jwt_rt_exp: Self::value("jwt_rt_exp"),
            dsn: Self::value("dsn"),
            root_email: Self::value("root_email"),
            root_password: Self::value("root_password"),
            root_dni: Self::value("root_dni"),
            root_social_reason: Self::value("root_social_reason"),
            environment: Self::value("environment"),
            jwt_aud: Self::value("jwt_aud"),
            jwt_kid: Self::value("jwt_kid"),
            support_email: Self::value("support_email"),
        }
    }
}

impl SecretsVariables {
    /// Reads the secret from /run/secrets/<key>. Returns None if not present.
    fn value(key: &str) -> String {
        let path = format!("/run/secrets/{}", key);
        match fs::read_to_string(&path) {
            Ok(value) => value.trim().to_string(),
            Err(_) => panic!("Secret '{}' is not defined in /run/secrets", key),
        }
    }
}
