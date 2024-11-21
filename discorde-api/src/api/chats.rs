use crate::api::DiscordeState;
use crate::chat::WsCommand;
use crate::models::chat::ChatInput;
use crate::models::user::User;
use axum::body::Body;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use futures_util::{SinkExt, StreamExt};
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::select;
use tokio::sync::broadcast::{Receiver, Sender};

#[axum::debug_handler]
async fn get_user_chats(
    Extension(user): Extension<User>,
    State(state): State<Arc<DiscordeState>>,
) -> Response<Body> {
    let mut cs = vec![];
    for chat in user.chats {
        let c = state.db.get_chat(chat.clone()).await.unwrap();
        if let Some(c) = c {
            cs.push(c.into_view(chat));
        }
    }

    Json(cs).into_response()
}

#[axum::debug_handler]
async fn create_chat(
    State(state): State<Arc<DiscordeState>>,
    Json(chat): Json<ChatInput>,
) -> StatusCode {
    let members = chat.members.clone();
    let chat = chat.into_chat();
    let id = state.db.insert_chat(chat).await.unwrap();

    for member in members {
        if let Some(mut member) = state.db.get_user(member).await.unwrap() {
            member.chats.push(id.clone());
            state.db.update_user(member).await.unwrap();
        }
    }

    StatusCode::CREATED
}

#[axum::debug_handler]
async fn get_chat_messages(
    State(state): State<Arc<DiscordeState>>,
    Path(chat): Path<String>,
) -> Json<BTreeSet<crate::models::chat::Message>> {
    Json(state.db.get_chat(chat).await.unwrap().unwrap().messages)
}

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[axum::debug_handler]
async fn ws_handler(
    Extension(user): Extension<User>,
    State(state): State<Arc<DiscordeState>>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(chat): Path<String>,
) -> impl IntoResponse {
    // Maybe auth
    let chat = state.chat.subscribe(chat).await;

    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, chat, user.username))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    (chat_tx, mut chat_rx): (Sender<WsCommand>, Receiver<WsCommand>),
    username: String,
) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Close(_) => return,
                _ => {}
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    tokio::spawn(async move {
        loop {
            select! {
                Some(Ok(msg)) = receiver.next() => {
                    match msg {
                        Message::Text(text) => {
                            if let Ok(cmd) = serde_json::from_str::<WsCommand>(&text) {
                                if cmd.from == username {
                                    match cmd.message {
                                        
                                    }
                                    _ = chat_tx.send(cmd);
                                }
                            }
                        }
                        Message::Close(_) => return,
                        _ => {},
                    }
                },
                Ok(msg) = chat_rx.recv() => {
                    if msg.from != username {
                        match msg.message {
                            
                        }
                        _ = sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await;
                    }
                }
            }
        }

        // let n_msg = 20;
        // for i in 0..n_msg {
        //     // In case of any websocket error, we exit.
        //     if sender
        //         .send(Message::Text(format!("Server message {i} ...")))
        //         .await
        //         .is_err()
        //     {
        //         return i;
        //     }
        //
        //     tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    });

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}

pub fn routes(state: Arc<DiscordeState>) -> Router<Arc<DiscordeState>> {
    Router::new()
        .route("/", post(create_chat))
        .route("/", get(get_user_chats))
        .route("/:id", get(ws_handler))
        .route("/:id/messages", get(get_chat_messages))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            super::middleware,
        ))
}
