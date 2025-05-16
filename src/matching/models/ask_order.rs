use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AskOrder {
    pub id: Uuid,
    pub quantity: u32,
    pub current_quantity: u32,
    pub price: u32,
}

impl Eq for AskOrder {}

impl PartialEq for AskOrder {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for AskOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.price.cmp(&self.price))
    }
}

impl Ord for AskOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        other.price.cmp(&self.price)
    }
}

impl AskOrder {
    pub const fn new(id: Uuid, quantity: u32, current_quantity: u32, price: u32) -> Self {
        Self {
            id,
            quantity,
            current_quantity,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let order = AskOrder::new(id, 200, 150, 300);

        assert_eq!(order.id, id);
        assert_eq!(order.quantity, 200);
        assert_eq!(order.current_quantity, 150);
        assert_eq!(order.price, 300);
    }

    #[test]
    fn test_edge_cases() {
        let max_order = AskOrder::new(Uuid::new_v4(), u32::MAX, u32::MAX, u32::MAX);
        assert_eq!(max_order.quantity, u32::MAX);

        let zero_order = AskOrder::new(Uuid::new_v4(), 0, 0, 0);
        assert_eq!(zero_order.price, 0);

        let extreme1 = AskOrder::new(Uuid::new_v4(), 1, 1, u32::MAX);
        let extreme2 = AskOrder::new(Uuid::new_v4(), u32::MAX, u32::MAX, u32::MAX);
        assert_eq!(extreme1, extreme2);
    }

    #[test]
    fn test_equality_based_on_price() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let order1 = AskOrder::new(id1, 100, 100, 50);
        let order2 = AskOrder::new(id2, 200, 200, 50);
        assert_eq!(order1, order2);

        let order3 = AskOrder::new(id1, 100, 100, 50);
        let order4 = AskOrder::new(id1, 100, 100, 55);
        assert_ne!(order3, order4);
    }

    #[test]
    fn test_partial_ordering() {
        let order1 = AskOrder::new(Uuid::new_v4(), 100, 100, 50);
        let order2 = AskOrder::new(Uuid::new_v4(), 200, 200, 60);

        assert_eq!(order1.partial_cmp(&order2), Some(Ordering::Greater));
        assert_eq!(order2.partial_cmp(&order1), Some(Ordering::Less));
        assert_eq!(order1.partial_cmp(&order1), Some(Ordering::Equal));
    }

    #[test]
    fn test_ordering() {
        let order_low = AskOrder::new(Uuid::new_v4(), 100, 100, 50);
        let order_high = AskOrder::new(Uuid::new_v4(), 100, 100, 60);

        assert_eq!(order_low.cmp(&order_high), Ordering::Greater);
        assert_eq!(order_high.cmp(&order_low), Ordering::Less);
    }

    #[test]
    fn test_sorting_same_price() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let order1 = AskOrder::new(id1, 100, 100, 50);
        let order2 = AskOrder::new(id2, 200, 200, 50);

        let mut orders = vec![order1.clone(), order2.clone()];
        orders.sort();
        assert_eq!(orders[0].id, order1.id);
        assert_eq!(orders[1].id, order2.id);
    }

    #[test]
    fn test_sorting() {
        let mut orders = vec![
            AskOrder::new(Uuid::new_v4(), 100, 100, 70),
            AskOrder::new(Uuid::new_v4(), 100, 100, 50),
            AskOrder::new(Uuid::new_v4(), 100, 100, 60),
        ];

        orders.sort();
        let prices: Vec<u32> = orders.iter().map(|o| o.price).collect();
        assert_eq!(prices, vec![70, 60, 50]);
    }

    #[test]
    fn test_opposite_ordering_from_ask_order() {
        use crate::matching::models::bid_order::BidOrder;

        let price_low = 50;
        let price_high = 60;

        let bid_low = BidOrder::new(Uuid::new_v4(), 100, 100, price_low);
        let bid_high = BidOrder::new(Uuid::new_v4(), 100, 100, price_high);

        let ask_low = AskOrder::new(Uuid::new_v4(), 100, 100, price_low);
        let ask_high = AskOrder::new(Uuid::new_v4(), 100, 100, price_high);

        let mut bids = vec![bid_low.clone(), bid_high.clone()];
        bids.sort();
        assert_eq!(bids[0].price, price_low);
        assert_eq!(bids[1].price, price_high);

        let mut asks = vec![ask_low.clone(), ask_high.clone()];
        asks.sort();
        assert_eq!(asks[0].price, price_high);
        assert_eq!(asks[1].price, price_low);
    }
}
