use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthResponse {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserResponse {
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
}


