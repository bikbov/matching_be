use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DepthOfMarket {
    pub ask: Vec<DepthEntry>,
    pub bid: Vec<DepthEntry>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DepthEntry {
    pub price: u32,
    pub quantity: u32,
}
