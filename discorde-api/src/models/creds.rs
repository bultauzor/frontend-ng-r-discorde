use crate::models::user::UserView;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
    pub token: String,
    pub user: UserView,
}
