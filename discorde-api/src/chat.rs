use crate::db::Database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{broadcast, oneshot};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WsMessage {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsCommand {
    pub from: String,
    pub message: WsMessage,
}

enum Command {
    Subscribe(
        String,
        oneshot::Sender<(broadcast::Sender<WsCommand>, broadcast::Receiver<WsCommand>)>,
    ),
}

pub struct ChatSvc {
    tx: UnboundedSender<Command>,
}

impl ChatSvc {
    pub fn new(db: Arc<Database>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
        tokio::spawn(Self::worker(rx, db));

        Self { tx }
    }

    async fn worker(mut comm_rx: UnboundedReceiver<Command>, db: Arc<Database>) {
        let mut chats: HashMap<
            String,
            (broadcast::Sender<WsCommand>, broadcast::Receiver<WsCommand>),
        > = HashMap::new();

        while let Some(command) = comm_rx.recv().await {
            match command {
                Command::Subscribe(chat_id, reply) => {
                    if let Some((tx, rx)) = chats.get(&chat_id) {
                        _ = reply.send((tx.clone(), rx.resubscribe())).unwrap();
                    } else {
                        chats.insert(chat_id, broadcast::channel(10));
                    }
                }
            }
        }
    }

    pub async fn subscribe(
        &self,
        chat_id: String,
    ) -> (broadcast::Sender<WsCommand>, broadcast::Receiver<WsCommand>) {
        let (tx, rx) = oneshot::channel();
        _ = self.tx.send(Command::Subscribe(chat_id, tx));
        rx.await.unwrap()
    }
}
