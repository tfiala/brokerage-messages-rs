use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeAccountLedgerPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeAccountsPayload {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribePositionsPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeTradesPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "id", content = "details")]
pub enum Subscription {
    #[serde(rename = "account-ledger")]
    AccountLedger(SubscribeAccountLedgerPayload),

    #[serde(rename = "accounts")]
    Accounts(SubscribeAccountsPayload),

    #[serde(rename = "positions")]
    Positions(SubscribePositionsPayload),

    #[serde(rename = "trades")]
    Trades(SubscribeTradesPayload),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectAccountPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "id", content = "details")]
pub enum ClientRequest {
    #[serde(rename = "select-account")]
    SelectAccount(SelectAccountPayload),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "proto", content = "data")]
pub enum ClientBrokerageProtocol {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "request")]
    Request(ClientRequest),
    #[serde(rename = "subscription")]
    Subscription(Subscription),
}
