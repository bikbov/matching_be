use crate::matching::models::ask_order::AskOrder;
use crate::matching::models::bid_order::BidOrder;
use crate::matching::models::dealbook::DealBook;
use crate::matching::models::depth_of_market::DepthOfMarket;
use crate::matching::models::order_message::OrderMessage;
use crate::matching::models::order_side::OrderSide;
use crate::matching::models::orderbook::OrderBook;
use crate::matching::send::send_data;
use tokio::sync::broadcast;

pub fn matching_engine(
    om_receiver: &mut broadcast::Receiver<OrderMessage>,
    dom_sender: &broadcast::Sender<DepthOfMarket>,
    db_sender: &broadcast::Sender<DealBook>,
) {
    let mut orderbook: OrderBook = OrderBook::new();
    while let Ok(order_message) = om_receiver.blocking_recv() {
        let mut dealbook: DealBook = DealBook::new();
        matching_orders(&order_message, &mut orderbook, &mut dealbook);
        send_data(dom_sender, orderbook.get_dom(), db_sender, dealbook);
    }
}

fn matching_orders(
    order_message: &OrderMessage,
    orderbook: &mut OrderBook,
    dealbook: &mut DealBook,
) {
    match order_message.side {
        OrderSide::Ask => {
            let ask_order = AskOrder::new(
                order_message.id,
                order_message.quantity,
                order_message.quantity,
                order_message.price,
            );
            let updated_ask_order = asks_match_bids(ask_order, orderbook, dealbook);
            orderbook.asks_push(updated_ask_order);
        }
        OrderSide::Bid => {
            let bid_order = BidOrder::new(
                order_message.id,
                order_message.quantity,
                order_message.quantity,
                order_message.price,
            );
            let updated_bid_order = bids_match_asks(bid_order, orderbook, dealbook);
            orderbook.bids_push(updated_bid_order);
        }
    }
}

fn asks_match_bids(
    mut ask_order: AskOrder,
    orderbook: &mut OrderBook,
    dealbook: &mut DealBook,
) -> AskOrder {
    while let Some(best_bid) = orderbook.bids.peek() {
        if best_bid.price < ask_order.price || ask_order.current_quantity == 0 {
            break;
        }

        if best_bid.current_quantity <= ask_order.current_quantity {
            ask_order = orderbook.bids_pop(ask_order, dealbook);
        } else {
            ask_order = orderbook.bids_peek_mut(ask_order, dealbook);
        }
    }

    AskOrder::new(
        ask_order.id,
        ask_order.quantity,
        ask_order.current_quantity,
        ask_order.price,
    )
}

fn bids_match_asks(
    mut bid_order: BidOrder,
    orderbook: &mut OrderBook,
    dealbook: &mut DealBook,
) -> BidOrder {
    while let Some(best_ask) = orderbook.asks.peek() {
        if best_ask.price > bid_order.price || bid_order.current_quantity == 0 {
            break;
        }

        if best_ask.current_quantity <= bid_order.current_quantity {
            bid_order = orderbook.asks_pop(bid_order, dealbook);
        } else {
            bid_order = orderbook.asks_peek_mut(bid_order, dealbook);
        }
    }

    BidOrder::new(
        bid_order.id,
        bid_order.quantity,
        bid_order.current_quantity,
        bid_order.price,
    )
}
