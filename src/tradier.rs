extern crate serde;

pub mod tradier {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaceOptionOrderParams {
        pub class: String,
        pub sym: String,
        pub option_symbol: String,
        pub side: String,
        pub quantity: i64,
        pub order_type: String,
        pub duration: String,
        pub price: Option<String>,
        pub stop: Option<String>,
        pub order_tag: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaceOptionOrder {
        pub id: i64,
        pub status: String,
        pub partner_id: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaceOptionOrderResponse {
        pub order: PlaceOptionOrder,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DeletedOrder {
        pub id: i64,
        pub status: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CancelOrderParams {
        pub order_id: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CancelOrderResponse {
        pub order: DeletedOrder,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OptionSymbols {
        pub rootSymbol: String,
        pub options: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct FetchAllOptionSymbolsResponse {
        pub symbols: Vec<OptionSymbols>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Greeks {
        pub delta: f64,
        pub gamma: f64,
        pub theta: f64,
        pub vega: f64,
        pub rho: f64,
        pub phi: f64,
        pub bid_iv: f64,
        pub mid_iv: f64,
        pub ask_iv: f64,
        pub smv_vol: f64,
        pub updated_at: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OptionSpotPrice {
        pub volume: Option<f64>,
        pub bid: Option<f64>,
        pub ask: Option<f64>,
        pub option_type: String,
        pub last_volume: Option<i64>,
        pub greeks: Greeks,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OptionListWrapper {
        pub option: Vec<OptionSpotPrice>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OptionsChain {
        pub options: OptionListWrapper,
    }
}
