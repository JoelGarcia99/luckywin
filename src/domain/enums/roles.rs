pub enum Roles {
    Admin,
    Client,
}

impl Roles {
    pub fn to_string(&self) -> String {
        match self {
            Roles::Admin => "admin".to_string(),
            Roles::Client => "client".to_string(),
        }
    }
}
