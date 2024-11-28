use crate::db::core::{Condition, Db, IdDocument};
use crate::models::chat::{Chat, Message};
use crate::models::user::User;
use serde_json::Value;
use std::io::Error;
use std::path::PathBuf;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tracing::{error, warn};

mod core;

enum Request {
    InsertUser(User, oneshot::Sender<Result<(), Error>>),
    GetUser(String, oneshot::Sender<Result<Option<User>, Error>>),
    GetUsers(oneshot::Sender<Vec<User>>),
    UpdateUser(User, oneshot::Sender<Result<(), Error>>),
    GetChat(String, oneshot::Sender<Result<Option<Chat>, Error>>),
    InsertChat(Chat, oneshot::Sender<Result<String, Error>>),
    InsertMessage(String, Message, oneshot::Sender<Result<(), Error>>),
}

pub struct Database(UnboundedSender<Request>);

impl Database {
    pub async fn new(path: PathBuf) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::task::spawn(Self::worker(path, rx));
        Self(tx)
    }

    async fn worker(path: PathBuf, mut rx: UnboundedReceiver<Request>) {
        let mut db = Db::new(path).await.map_err(|error| error!(?error)).unwrap();
        db.lock().await.map_err(|error| error!(?error)).unwrap();

        while let Some(req) = rx.recv().await {
            match req {
                Request::InsertUser(user, reply) => {
                    let res = db.clone().collection("users").add(user).await.map(|_| ());
                    _ = reply.send(res);
                }
                Request::GetUser(id, reply) => {
                    let res = db
                        .clone()
                        .collection("users")
                        .wherr("username".to_string(), Condition::Equal, Value::String(id))
                        .await
                        .unwrap()
                        .get()
                        .first()
                        .cloned();
                    let res = match res {
                        None => Ok(None),
                        Some(doc) => doc.doc.get().await,
                    };
                    _ = reply.send(res);
                }
                Request::GetUsers(reply) => {
                    let mut res = vec![];
                    for doc in db.clone().collection("users").get().await {
                        match doc.doc.get().await {
                            Ok(Some(user)) => res.push(user),
                            Ok(None) => warn!("Failed to retrieve doc"),
                            Err(error) => error!(?error),
                        }
                    }
                    _ = reply.send(res);
                }
                Request::GetChat(id, reply) => {
                    let res = db.clone().collection("chats").doc(&id).get().await;
                    _ = reply.send(res);
                }
                Request::InsertChat(chat, reply) => {
                    let res = db.clone().collection("chats").add(chat).await;
                    _ = reply.send(res);
                }
                Request::UpdateUser(user, reply) => {
                    let res = db
                        .clone()
                        .collection("users")
                        .wherr(
                            "username".to_string(),
                            Condition::Equal,
                            Value::String(user.username.clone()),
                        )
                        .await
                        .unwrap()
                        .get()
                        .first()
                        .cloned();
                    let res = match res {
                        None => continue,
                        Some(mut doc) => doc.doc.update(user).await,
                    };

                    _ = reply.send(res);
                }
                Request::InsertMessage(id, message, reply) => {
                    let res = db.clone().collection("chats").doc(&id).get::<Chat>().await;
                    let res = match res {
                        Ok(Some(mut chat)) => {
                            chat.messages.insert(message);
                            db.clone().collection("chats").doc(&id).update(chat).await
                        }
                        Err(error) => Err(error),
                        Ok(None) => Ok(())
                    };
                    _ = reply.send(res);
                }
            }
        }
    }

    pub async fn insert_user(&self, user: User) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::InsertUser(user, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn update_user(&self, user: User) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::UpdateUser(user, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn get_user(&self, user: String) -> Result<Option<User>, Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::GetUser(user, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn get_users(&self) -> Vec<User> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::GetUsers(tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn get_chat(&self, chat: String) -> Result<Option<Chat>, Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::GetChat(chat, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn insert_chat(&self, chat: Chat) -> Result<String, Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::InsertChat(chat, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }

    pub async fn insert_message(&self, chat: String, message: Message) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        _ = self.0.send(Request::InsertMessage(chat, message, tx));
        rx.await.map_err(|error| error!(?error)).unwrap()
    }
}
