use crate::matching::models::order_side::OrderSide;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct OrderMessage {
    pub id: Uuid,
    pub side: OrderSide,
    pub quantity: u32,
    pub price: u32,
}
