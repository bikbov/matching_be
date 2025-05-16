mod handlers;
mod matching;
use crate::matching::models::dealbook::DealBook;
use crate::matching::models::depth_of_market::DepthOfMarket;
use crate::matching::models::order_message::OrderMessage;
use axum::{Router, routing::any, routing::get, routing::post};
use handlers::{create_order, get_deals, get_orderbook, healthcheck};
use matching::engine::matching_engine;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::spawn_blocking;

#[derive(Clone)]
pub struct AppState {
    order_message_sender: Arc<broadcast::Sender<OrderMessage>>,
    orderbook_receiver: Arc<broadcast::Receiver<DepthOfMarket>>,
    dealbook_receiver: Arc<broadcast::Receiver<DealBook>>,
}

#[tokio::main]
async fn main() {
    let port = 28103_u16;
    let addr_size = 1000_usize;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let (om_sender, _) = broadcast::channel(addr_size);
    let (dom_sender, _) = broadcast::channel(addr_size);
    let (db_sender, _) = broadcast::channel(addr_size);

    let mut om_receiver: broadcast::Receiver<OrderMessage> = om_sender.subscribe();
    let dom_receiver: broadcast::Receiver<DepthOfMarket> = dom_sender.subscribe();
    let db_receiver: broadcast::Receiver<DealBook> = db_sender.subscribe();

    let state: AppState = AppState {
        order_message_sender: Arc::new(om_sender),
        orderbook_receiver: Arc::new(dom_receiver),
        dealbook_receiver: Arc::new(db_receiver),
    };

    spawn_blocking(move || matching_engine(&mut om_receiver, &dom_sender, &db_sender));

    let app = Router::new()
        .route("/api/orderbook", any(get_orderbook))
        .route("/api/dealbook", any(get_deals))
        .route("/api/orders", post(create_order))
        .route("/api/health", get(healthcheck))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
