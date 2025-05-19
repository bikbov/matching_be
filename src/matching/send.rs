use crate::DealBook;
use crate::DepthOfMarket;
use tokio::sync::broadcast;

pub fn send_data(
    dom_sender: &broadcast::Sender<DepthOfMarket>,
    orderbook: DepthOfMarket,
    db_sender: &broadcast::Sender<DealBook>,
    dealbook: DealBook,
) {
    send_deals(db_sender, dealbook);
    send_orderbook(dom_sender, orderbook);
}

fn send_deals(db_sender: &broadcast::Sender<DealBook>, dealbook: DealBook) {
    if !dealbook.deals.is_empty() && db_sender.send(dealbook).is_err() {
        println!("Error_dealbook");
    }
}

fn send_orderbook(dom_sender: &broadcast::Sender<DepthOfMarket>, orderbook: DepthOfMarket) {
    if dom_sender.send(orderbook).is_err() {
        println!("Error_orderbook");
    }
}
