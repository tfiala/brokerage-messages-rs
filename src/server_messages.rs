use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]

pub struct ServerMessage<'a, T> {
    proto: &'a str,
    data: T,
}

impl<'a, T> ServerMessage<'a, T> {
    pub fn new(proto: &'a str, data: T) -> Self {
        Self { proto, data }
    }

    pub fn new_push(data: T) -> Self {
        Self {
            proto: "push",
            data,
        }
    }

    pub fn new_request(data: T) -> Self {
        Self {
            proto: "request",
            data,
        }
    }

    pub fn new_subscription(data: T) -> Self {
        Self {
            proto: "subscription",
            data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]

pub struct ServerRequest<'a, T> {
    request_id: String,
    id: &'a str,
    details: T,
}

impl<'a, T> ServerRequest<'a, T> {
    pub fn new(request_id: String, id: &'a str, details: T) -> Self {
        Self {
            request_id,
            id,
            details,
        }
    }
}

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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SelectAccountDetails {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}

pub fn make_select_account_request<'a>(
    request_id: String,
    account_id: String,
    brokerage_id: String,
) -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    ServerMessage::new_request(ServerRequest::new(
        request_id,
        "select-account",
        SelectAccountDetails {
            account_id,
            brokerage_id,
        },
    ))
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "id", content = "details")]
// pub enum ClientRequest {
//     #[serde(rename = "select-account")]
//     SelectAccount(SelectAccountPayload),
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "proto", content = "data")]
// pub enum ClientBrokerageProtocol {
//     #[serde(rename = "push")]
//     Push,
//     #[serde(rename = "request")]
//     Request(ClientRequest),
//     #[serde(rename = "subscription")]
//     Subscription(Subscription),
// }
