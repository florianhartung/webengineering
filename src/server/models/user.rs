use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    // TODO don't send password to client
    // #[cfg_attr(feature = "hydrate", allow(dead_code))]
    // #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    pub fn new_validated(username: String, password: String) -> Result<Self, String> {
        if password.len() < 8 {
            return Err("Password must be atleast 8 characters long".to_owned());
        }

        Ok(Self {
            username,
            password,
        })
    }
}