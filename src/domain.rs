use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct BrokerageAccount {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AccountLedger {
    #[serde(rename = "account-id")]
    pub account_id: String,

    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,

    pub account_type: String,
    pub available_funds: f64,
    pub equity_with_loan_value: f64,
    pub gross_position_value: f64,
    pub cash_balance: f64,
}
