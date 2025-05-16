use crate::matching::models::deal::Deal;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DealBook {
    pub deals: Vec<Deal>,
}

impl DealBook {
    pub const fn new() -> Self {
        Self { deals: Vec::new() }
    }

    pub fn push(
        &mut self,
        bid_order_price: u32,
        deal_quantity: u32,
        ask_order_id: Uuid,
        bid_order_id: Uuid,
    ) {
        let new_deal: Deal = Deal {
            time: Utc::now(),
            price: bid_order_price,
            quantity: deal_quantity,
            ask_order: ask_order_id,
            bid_order: bid_order_id,
        };

        self.deals.push(new_deal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_empty_deal_book() {
        let deal_book = DealBook { deals: Vec::new() };

        assert_eq!(deal_book.deals.len(), 0);
        assert!(deal_book.deals.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        let mut deal_book = DealBook { deals: Vec::new() };
        let uuid = Uuid::new_v4();

        deal_book.push(0, 0, uuid, uuid);
        assert_eq!(deal_book.deals[0].quantity, 0);

        deal_book.push(u32::MAX, u32::MAX, uuid, uuid);
        assert_eq!(deal_book.deals[1].price, u32::MAX);
    }

    #[test]
    fn test_push_adds_valid_deal() {
        let mut deal_book = DealBook { deals: Vec::new() };
        let ask_uuid = Uuid::new_v4();
        let bid_uuid = Uuid::new_v4();

        let price = 1500;
        let quantity = 30;

        let time_before = Utc::now();
        deal_book.push(price, quantity, ask_uuid, bid_uuid);
        let time_after = Utc::now();

        assert_eq!(deal_book.deals.len(), 1);

        let deal = &deal_book.deals[0];
        assert_eq!(deal.price, price);
        assert_eq!(deal.quantity, quantity);
        assert_eq!(deal.ask_order, ask_uuid);
        assert_eq!(deal.bid_order, bid_uuid);

        assert!(
            deal.time >= time_before && deal.time <= time_after,
            "Deal time should be between time_before and time_after"
        );
    }

    #[test]
    fn test_multiple_pushes() {
        let mut deal_book = DealBook { deals: Vec::new() };
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();

        deal_book.push(100, 10, uuid1, uuid2);
        assert_eq!(deal_book.deals.len(), 1);

        deal_book.push(200, 20, uuid2, uuid1);
        assert_eq!(deal_book.deals.len(), 2);

        let last_deal = deal_book.deals.last().unwrap();
        assert_eq!(last_deal.price, 200);
        assert_eq!(last_deal.quantity, 20);
        assert_eq!(last_deal.ask_order, uuid2);
        assert_eq!(last_deal.bid_order, uuid1);
    }
}
