use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

impl UserInput {
    pub fn into_user(self) -> User {
        User {
            username: self.username,
            password: self.password,
            chats: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub chats: Vec<String>,
}

impl User {
    pub fn into_view(self) -> UserView {
        UserView {
            username: self.username,
            chats: self.chats,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserView {
    pub username: String,
    pub chats: Vec<String>,
}
