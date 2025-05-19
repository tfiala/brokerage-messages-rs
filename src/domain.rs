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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct EndOfDaySummary {
    pub account_id: String,
    pub brokerage_id: String,

    pub net_cash_balance: f64,
    pub net_stock_balance: f64,

    pub interest: f64,
    pub interest_mtd: f64,

    pub dividends: f64,
    pub dividends_mtd: f64,

    pub commissions: f64,
    pub commissions_mtd: f64,

    pub other_fees: f64,
    pub other_fees_mtd: f64,
}
