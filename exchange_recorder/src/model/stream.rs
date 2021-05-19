#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetfairMessage {
    pub op: String,
    pub id: Option<i64>,
    pub initial_clk: Option<String>,
    pub clk: Option<String>,
    pub conflate_ms: Option<i64>,
    pub heartbeat_ms: Option<i64>,
    pub pt: Option<i64>,
    pub ct: Option<String>,
    pub mc: Option<Vec<Mc>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mc {
    pub id: String,
    pub market_definition: Option<MarketDefinition>,
    pub rc: Option<Vec<Rc>>,
    pub img: Option<bool>,
    pub tv: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDefinition {
    pub bsp_market: bool,
    pub turn_in_play_enabled: bool,
    pub persistence_enabled: bool,
    pub market_base_rate: i64,
    pub event_id: String,
    pub event_type_id: String,
    pub number_of_winners: i64,
    pub betting_type: String,
    pub market_type: String,
    pub market_time: String,
    pub suspend_time: String,
    pub bsp_reconciled: bool,
    pub complete: bool,
    pub in_play: bool,
    pub cross_matching: bool,
    pub runners_voidable: bool,
    pub number_of_active_runners: i64,
    pub bet_delay: i64,
    pub status: String,
    pub runners: Vec<Runner>,
    pub regulators: Vec<String>,
    pub venue: String,
    pub country_code: String,
    pub discount_allowed: bool,
    pub timezone: String,
    pub open_date: String,
    pub version: i64,
    pub race_type: String,
    pub price_ladder_definition: PriceLadderDefinition,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runner {
    pub adjustment_factor: f64,
    pub status: String,
    pub sort_priority: i64,
    pub id: i64,
    pub removal_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLadderDefinition {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rc {
    #[serde(default)]
    pub atb: Vec<Vec<f64>>,
    #[serde(default)]
    pub atl: Vec<Vec<f64>>,
    #[serde(default)]
    pub trd: Vec<Vec<f64>>,
    #[serde(default)]
    pub spb: Vec<Vec<f64>>,
    #[serde(default)]
    pub spl: Vec<Vec<f64>>,
    pub batb: Option<Vec<Vec<f64>>>,
    #[serde(default)]
    pub batl: Vec<Vec<f64>>,
    pub bdatb: Option<Vec<Vec<f64>>>,
    pub bdatl: Option<Vec<Vec<f64>>>,
    pub spn: Option<f64>,
    pub spf: Option<f64>,
    pub ltp: Option<f64>,
    pub tv: Option<f64>,
    pub id: i64,
}
