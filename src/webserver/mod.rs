use std::{net::SocketAddr, sync::Mutex};

use axum::{
    body::Bytes,
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};

#[derive(Clone)]
pub struct AppState {
    // pub next_id: std::sync::Arc<AtomicU64>,
    pub value: u32,
}

pub async fn websocket_handler(State(state): State<AppState>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    //let id = state.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // println!("`{user_agent}` at {addr} connected.");
    ws.on_failed_upgrade(|error| println!("Error upgrading websocket: {}", error))
        .on_upgrade(move |socket| handle_web_socket(socket, addr))
}

async fn send_close_message(mut sender: SplitSink<WebSocket, Message>, code: u16, reason: &str) {
    _ = sender
        .send(Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        })))
        .await;
}

async fn handle_web_socket(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }
    // TODO: Returns `None` if the stream has closed.
    // debugging websocket connection

    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = receiver.next().await {
        // TODO: process messages in extra function
        if let Ok(msg) = msg {
            match msg {
                Message::Text(utf8_bytes) => {
                    println!("Text received: {}", utf8_bytes);
                    let result = sender
                        .send(Message::Text(
                            format!("Echo back text: {}", utf8_bytes).into(),
                        ))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(sender, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                Message::Binary(bytes) => {
                    println!("Received bytes of length: {}", bytes.len());
                    let result = sender
                        .send(Message::Text(
                            format!("Received bytes of length: {}", bytes.len()).into(),
                        ))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(sender, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                Message::Close(c) => {
                    if let Some(cf) = c {
                        println!(
                            ">>> {who} sent close with code {} and reason `{}`",
                            cf.code, cf.reason
                        );
                    } else {
                        println!(">>> {who} somehow sent close message without CloseFrame");
                    }
                    break;
                }
                _ => {}
            }
        } else {
            let error = msg.err().unwrap();
            println!("Error receiving message: {:?}", error);
            // send_close_message(socket, 1011, &format!("Error occured: {}", error)).await;
            break;
        }
    }
}
