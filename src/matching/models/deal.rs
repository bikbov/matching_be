use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Deal {
    pub time: DateTime<Utc>,
    pub price: u32,
    pub quantity: u32,
    pub ask_order: Uuid,
    pub bid_order: Uuid,
}
