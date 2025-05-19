use crate::DepthOfMarket;
use crate::matching::models::ask_order::AskOrder;
use crate::matching::models::bid_order::BidOrder;
use crate::matching::models::dealbook::DealBook;
use crate::matching::models::depth_of_market::DepthEntry;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct OrderBook {
    pub asks_book: HashMap<u32, u32>, //используется для быстрого показа стакана
    pub bids_book: HashMap<u32, u32>,
    pub asks: BinaryHeap<AskOrder>, //используется для быстрого исполнения сделок
    pub bids: BinaryHeap<BidOrder>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            asks_book: HashMap::new(),
            bids_book: HashMap::new(),
            asks: BinaryHeap::new(),
            bids: BinaryHeap::new(),
        }
    }

    fn add_asks_book_quantity(&mut self, deal_quantity: u32, price: u32) {
        let entry = self.asks_book.entry(price).or_insert(0);
        *entry += deal_quantity;
    }

    fn add_bids_book_quantity(&mut self, deal_quantity: u32, price: u32) {
        let entry = self.bids_book.entry(price).or_insert(0);
        *entry += deal_quantity;
    }

    fn subtract_asks_book_quantity(&mut self, deal_quantity: u32, price: u32) {
        if let Some(quantity) = self.asks_book.get_mut(&price) {
            if *quantity <= deal_quantity {
                self.asks_book.remove(&price);
            } else {
                *quantity -= deal_quantity;
            }
        }
    }

    fn subtract_bids_book_quantity(&mut self, deal_quantity: u32, price: u32) {
        if let Some(quantity) = self.bids_book.get_mut(&price) {
            if *quantity <= deal_quantity {
                self.bids_book.remove(&price);
            } else {
                *quantity -= deal_quantity;
            }
        }
    }

    pub fn asks_push(&mut self, ask_order: AskOrder) {
        if ask_order.current_quantity > 0 {
            Self::add_asks_book_quantity(self, ask_order.current_quantity, ask_order.price);
            self.asks.push(ask_order);
        }
    }

    pub fn bids_push(&mut self, bid_order: BidOrder) {
        if bid_order.current_quantity > 0 {
            Self::add_bids_book_quantity(self, bid_order.current_quantity, bid_order.price);
            self.bids.push(bid_order);
        }
    }

    ///метод запускается когда есть бид, после которого нужно вытащить аск-ордер, об который он погасится
    pub fn asks_pop(&mut self, bid_order: BidOrder, dealbook: &mut DealBook) -> BidOrder {
        if let Some(ask_order) = self.asks.pop() {
            Self::subtract_asks_book_quantity(self, ask_order.current_quantity, ask_order.price);
            dealbook.push(
                ask_order.price,
                ask_order.current_quantity,
                ask_order.id,
                bid_order.id,
            );

            return BidOrder::new(
                bid_order.id,
                bid_order.quantity,
                bid_order.current_quantity - ask_order.current_quantity,
                bid_order.price,
            );
        }
        bid_order
    }

    ///метод запускается когда есть аск, после которого нужно вытащить бид-ордер, об который он погасится
    pub fn bids_pop(&mut self, ask_order: AskOrder, dealbook: &mut DealBook) -> AskOrder {
        if let Some(bid_order) = self.bids.pop() {
            Self::subtract_bids_book_quantity(self, bid_order.current_quantity, bid_order.price);
            dealbook.push(
                bid_order.price,
                bid_order.current_quantity,
                ask_order.id,
                bid_order.id,
            );

            return AskOrder::new(
                ask_order.id,
                ask_order.quantity,
                ask_order.current_quantity - bid_order.current_quantity,
                ask_order.price,
            );
        }
        ask_order
    }

    ///метод запускается когда есть бид, после которого нужно изменить аск-ордер, об который он погасится
    pub fn asks_peek_mut(&mut self, bid_order: BidOrder, dealbook: &mut DealBook) -> BidOrder {
        if let Some(mut ask_order) = self.asks.peek_mut() {
            ask_order.current_quantity -= bid_order.current_quantity;

            if let Some(quantity) = self.asks_book.get_mut(&ask_order.price) {
                *quantity -= bid_order.current_quantity;
            }

            dealbook.push(
                ask_order.price,
                bid_order.current_quantity,
                ask_order.id,
                bid_order.id,
            );

            return BidOrder::new(bid_order.id, bid_order.quantity, 0_u32, bid_order.price);
        }

        bid_order
    }

    ///метод запускается когда есть аск, после которого нужно изменить бид-ордер, об который он погасится
    pub fn bids_peek_mut(&mut self, ask_order: AskOrder, dealbook: &mut DealBook) -> AskOrder {
        if let Some(mut bid_order) = self.bids.peek_mut() {
            bid_order.current_quantity -= ask_order.current_quantity;

            if let Some(quantity) = self.bids_book.get_mut(&bid_order.price) {
                *quantity -= ask_order.current_quantity;
            }

            dealbook.push(
                bid_order.price,
                ask_order.current_quantity,
                ask_order.id,
                bid_order.id,
            );
            return AskOrder::new(ask_order.id, ask_order.quantity, 0_u32, ask_order.price);
        }

        ask_order
    }

    pub fn get_dom(&self) -> DepthOfMarket {
        let ask: Vec<DepthEntry> = self
            .asks_book
            .iter()
            .map(|(&price, &quantity)| DepthEntry { price, quantity })
            .collect();

        let bid: Vec<DepthEntry> = self
            .bids_book
            .iter()
            .map(|(&price, &quantity)| DepthEntry { price, quantity })
            .collect();

        DepthOfMarket { ask, bid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching::models::ask_order::AskOrder;
    use crate::matching::models::bid_order::BidOrder;
    use uuid::Uuid;

    // классы эквивалентности  asks_push, bids_push

    // Для current_quantity ордера:
    // current_quantity = 0
    // current_quantity > 0,

    // Для цены ордера:
    // Первый ордер по новой цене
    // Ордер по существующей цене

    #[test]
    fn test_asks_push_zero_current_quantity() {
        let mut orderbook = OrderBook::new();
        let ask_id = Uuid::new_v4();
        let ask_order = AskOrder::new(ask_id, 100, 0, 500);

        orderbook.asks_push(ask_order);

        assert_eq!(orderbook.asks.len(), 0);
        assert!(&orderbook.asks_book.is_empty());
    }

    #[test]
    fn test_asks_push_some_current_quantity() {
        let mut orderbook = OrderBook::new();
        let ask_id = Uuid::new_v4();
        let ask_order = AskOrder::new(ask_id, 100, 1, 500);

        orderbook.asks_push(ask_order);

        assert_eq!(orderbook.asks.len(), 1);
        assert!(!&orderbook.asks_book.is_empty());
    }

    #[test]
    fn test_asks_push_existed_price() {
        let mut orderbook = OrderBook::new();
        let ask_id1 = Uuid::new_v4();
        let price: u32 = 500;

        let ask_order1 = AskOrder::new(ask_id1, 100, 1, price);
        orderbook.asks_push(ask_order1);

        let ask_id2 = Uuid::new_v4();
        let ask_order2 = AskOrder::new(ask_id2, 100, 2, price);
        orderbook.asks_push(ask_order2);

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.asks_book.get(&price).unwrap(), &3);
    }

    #[test]
    fn test_bids_push_zero_current_quantity() {
        let mut orderbook = OrderBook::new();
        let bid_id = Uuid::new_v4();
        let bid_order = BidOrder::new(bid_id, 100, 0, 500);

        orderbook.bids_push(bid_order);

        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.bids_book.is_empty());
    }

    #[test]
    fn test_bids_push_some_current_quantity() {
        let mut orderbook = OrderBook::new();
        let bid_id = Uuid::new_v4();
        let bid_order = BidOrder::new(bid_id, 100, 1, 500);

        orderbook.bids_push(bid_order);

        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.bids_book.is_empty());
    }

    #[test]
    fn test_bids_push_existed_price() {
        let mut orderbook = OrderBook::new();
        let price: u32 = 500;

        let bid_id1 = Uuid::new_v4();
        let bid_order1 = BidOrder::new(bid_id1, 100, 1, price);
        orderbook.bids_push(bid_order1);

        let bid_id2 = Uuid::new_v4();
        let bid_order2 = BidOrder::new(bid_id2, 100, 2, price);
        orderbook.bids_push(bid_order2);

        assert_eq!(orderbook.bids.len(), 2);
        assert_eq!(orderbook.bids_book.get(&price).unwrap(), &3);
    }

    //Классы эквивалентности asks_pop, bids_pop
    // Количество:
    // 1. Очередь asks/bids пуста
    // 2. Очередь asks/bids непуста

    #[test]
    fn test_asks_pop_empty_asks_queue() {
        let mut order_book = OrderBook::new();
        let mut dealbook = DealBook::new();
        let bid_id = Uuid::new_v4();
        let bid_order = BidOrder::new(bid_id, 100, 50, 500);
        let result_order = order_book.asks_pop(bid_order, &mut dealbook);

        assert_eq!(result_order.price, 500);
        assert_eq!(result_order.quantity, 100);
        assert_eq!(result_order.current_quantity, 50);
        assert!(dealbook.deals.is_empty());
    }

    #[test]
    fn test_asks_pop_non_empty_asks_queue() {
        let mut order_book = OrderBook::new();
        let mut dealbook = DealBook::new();
        let bid_id = Uuid::new_v4();
        let price: u32 = 500;
        let ask_order = AskOrder::new(bid_id, 100, 50, price);
        order_book.asks_push(ask_order);

        let bid_id = Uuid::new_v4();
        let bid_order = BidOrder::new(bid_id, 100, 100, price);
        let result_order = order_book.asks_pop(bid_order, &mut dealbook);

        assert_eq!(result_order.price, 500);
        assert_eq!(result_order.quantity, 100);
        assert_eq!(result_order.current_quantity, 50);

        assert!(order_book.asks.is_empty());
        assert!(!dealbook.deals.is_empty());
    }

    #[test]
    fn test_bids_pop_empty_bids_queue() {
        let mut order_book = OrderBook::new();
        let mut dealbook = DealBook::new();
        let ask_id = Uuid::new_v4();
        let ask_order = AskOrder::new(ask_id, 100, 50, 500);
        let result_order = order_book.bids_pop(ask_order, &mut dealbook);

        assert_eq!(result_order.price, 500);
        assert_eq!(result_order.quantity, 100);
        assert_eq!(result_order.current_quantity, 50);
        assert!(dealbook.deals.is_empty());
    }

    #[test]
    fn test_bids_pop_non_empty_bids_queue() {
        let mut order_book = OrderBook::new();
        let mut dealbook = DealBook::new();
        let ask_id = Uuid::new_v4();
        let bid_order = BidOrder::new(ask_id, 100, 50, 500);
        order_book.bids_push(bid_order);

        let bid_id = Uuid::new_v4();
        let ask_order = AskOrder::new(bid_id, 100, 100, 500);
        let result_order = order_book.bids_pop(ask_order, &mut dealbook);

        assert_eq!(result_order.price, 500);
        assert_eq!(result_order.quantity, 100);
        assert_eq!(result_order.current_quantity, 50);

        assert!(order_book.bids.is_empty());
        assert!(!dealbook.deals.is_empty());
    }

    //Классы эквивалентности asks_peek_mut, bids_peek_mut
    // Количество:
    // 1. Очередь asks/bids пуста
    // 2. Очередь asks/bids непуста

    #[test]
    fn test_get_dom() {
        let mut orderbook = OrderBook::new();
        orderbook.asks_push(AskOrder::new(Uuid::new_v4(), 100, 100, 510));
        orderbook.asks_push(AskOrder::new(Uuid::new_v4(), 50, 50, 500));
        orderbook.bids_push(BidOrder::new(Uuid::new_v4(), 70, 70, 490));
        orderbook.bids_push(BidOrder::new(Uuid::new_v4(), 30, 30, 480));

        let dom = orderbook.get_dom();

        assert_eq!(dom.ask.len(), 2);

        let ask_prices: Vec<u32> = dom.ask.iter().map(|entry| entry.price).collect();
        assert!(ask_prices.contains(&500));
        assert!(ask_prices.contains(&510));
        assert_eq!(dom.bid.len(), 2);

        let bid_prices: Vec<u32> = dom.bid.iter().map(|entry| entry.price).collect();
        assert!(bid_prices.contains(&480));
        assert!(bid_prices.contains(&490));

        for entry in dom.ask {
            if entry.price == 500 {
                assert_eq!(entry.quantity, 50);
            } else if entry.price == 510 {
                assert_eq!(entry.quantity, 100);
            }
        }

        for entry in dom.bid {
            if entry.price == 480 {
                assert_eq!(entry.quantity, 30);
            } else if entry.price == 490 {
                assert_eq!(entry.quantity, 70);
            }
        }
    }
}
