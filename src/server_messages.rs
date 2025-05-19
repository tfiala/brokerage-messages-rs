use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerMessage<'a, T> {
    pub proto: &'a str,
    pub data: T,
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
    pub request_id: String,
    pub id: &'a str,
    pub details: T,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerSubscription<'a, T> {
    pub id: &'a str,
    pub details: T,
}

impl<'a, T> ServerSubscription<'a, T> {
    pub fn new(id: &'a str, details: T) -> Self {
        Self { id, details }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscribeAccountsDetails {}

pub fn make_subscribe_accounts_message<'a>()
-> ServerMessage<'a, ServerSubscription<'a, SubscribeAccountsDetails>> {
    ServerMessage::new_subscription(ServerSubscription::new(
        "accounts",
        SubscribeAccountsDetails {},
    ))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscribeAccountLedgerDetails {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}

pub fn make_subscribe_account_ledger_message<'a>(
    account_id: String,
    brokerage_id: String,
) -> ServerMessage<'a, ServerSubscription<'a, SubscribeAccountLedgerDetails>> {
    ServerMessage::new_subscription(ServerSubscription::new(
        "account-ledger",
        SubscribeAccountLedgerDetails {
            account_id,
            brokerage_id,
        },
    ))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscribePositionsPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscribeTradesPayload {
    #[serde(rename = "account-id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "details")]
pub enum Subscription {
    #[serde(rename = "account-ledger")]
    AccountLedger(SubscribeAccountLedgerDetails),

    #[serde(rename = "accounts")]
    Accounts(SubscribeAccountsDetails),

    #[serde(rename = "positions")]
    Positions(SubscribePositionsPayload),

    #[serde(rename = "trades")]
    Trades(SubscribeTradesPayload),
}

//
// Select Account Request
//

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

//
// End of Day Summary Request
//

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EndOfDaySummaryDetails {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
}

pub fn make_end_of_day_summary_request<'a>(
    request_id: String,
    account_id: String,
    brokerage_id: String,
) -> ServerMessage<'a, ServerRequest<'a, EndOfDaySummaryDetails>> {
    ServerMessage::new_request(ServerRequest::new(
        request_id,
        "eod-summary",
        EndOfDaySummaryDetails {
            account_id,
            brokerage_id,
        },
    ))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "details")]
pub enum EnumRequestDetails {
    #[serde(rename = "select-account")]
    SelectAccount(SelectAccountDetails),

    #[serde(rename = "eod-summary")]
    EndOfDaySummary(EndOfDaySummaryDetails),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "details")]
pub enum EnumSubDetails {
    #[serde(rename = "accounts")]
    Accounts(SubscribeAccountsDetails),
    #[serde(rename = "account-ledger")]
    AccountLedger(SubscribeAccountLedgerDetails),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "proto", content = "data")]
pub enum EnumMessage {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "request")]
    Request(EnumRequestDetails),
    #[serde(rename = "subscription")]
    Subscription(EnumSubDetails),
}
