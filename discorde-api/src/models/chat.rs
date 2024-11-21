use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Deserialize)]
pub struct ChatInput {
    pub private: bool,
    pub name: String,
    pub members: Vec<String>,
}

impl ChatInput {
    pub fn into_chat(self) -> Chat {
        Chat {
            private: self.private,
            name: self.name,
            members: self.members,
            messages: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub private: bool,
    pub name: String,
    pub members: Vec<String>,
    pub messages: BTreeSet<Message>,
}

impl Chat {
    pub fn into_view(self, id: String) -> ChatView {
        ChatView {
            id,
            private: self.private,
            name: self.name,
            members: self.members,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: u64,
    pub author: String,
    pub message: String,
}

impl PartialEq<Self> for Message {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Message {}

impl PartialOrd<Self> for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

#[derive(Debug, Serialize)]
pub struct ChatView {
    id: String,
    pub private: bool,
    pub name: String,
    pub members: Vec<String>,
}
