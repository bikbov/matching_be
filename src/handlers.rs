use crate::AppState;
use crate::matching::models::order_message::OrderMessage;
use axum::{
    Json,
    extract::State,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    response::Response,
};

pub async fn healthcheck() -> impl IntoResponse {
    (axum::http::StatusCode::OK,)
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(message): Json<OrderMessage>,
) -> impl IntoResponse {
    let _ = (*state.order_message_sender).send(message);
    (axum::http::StatusCode::CREATED,)
}

pub async fn get_orderbook(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_orderbook(socket, state))
}

async fn handle_orderbook(mut socket: WebSocket, state: AppState) {
    loop {
        let mut dom_receiver = (*state.orderbook_receiver).resubscribe();
        match dom_receiver.recv().await {
            Ok(dom) => match serde_json::to_string(&dom) {
                Ok(json_string) => {
                    if let Err(e) = socket.send(json_string.into()).await {
                        eprintln!("Error sending orderbook: {e:?}");
                    }
                }
                Err(e) => {
                    eprintln!("Error serializing orderbook to JSON: {e:?}");
                }
            },
            Err(e) => {
                eprintln!("Error receiving orderbook: {e:?}");
            }
        }
    }
}

pub async fn get_deals(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_deals(socket, state))
}

async fn handle_deals(mut socket: WebSocket, state: AppState) {
    loop {
        let mut db_receiver = (*state.dealbook_receiver).resubscribe();
        match db_receiver.recv().await {
            Ok(dealbook) => match serde_json::to_string(&dealbook.deals) {
                Ok(json_string) => {
                    if let Err(e) = socket.send(json_string.into()).await {
                        eprintln!("Error sending dealbook: {e:?}");
                    }
                }
                Err(e) => {
                    eprintln!("Error serializing dealbook to JSON: {e:?}");
                }
            },
            Err(e) => {
                eprintln!("Error receiving dealbook: {e:?}");
            }
        }
    }
}
