use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct BrokerageAccount {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}
