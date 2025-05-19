use super::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]

pub struct ClientMessage<'a, T> {
    proto: &'a str,
    data: T,
}

impl<'a, T> ClientMessage<'a, T> {
    pub fn new(proto: &'a str, data: T) -> Self {
        Self { proto, data }
    }

    pub fn new_response(data: T) -> Self {
        Self {
            proto: "response",
            data,
        }
    }

    pub fn new_subscription_update(data: T) -> Self {
        Self {
            proto: "subscription",
            data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClientResponse<'a, T> {
    #[serde(rename = "request-id")]
    request_id: String,
    id: &'a str,
    details: T,
}

impl<'a, T> ClientResponse<'a, T> {
    pub fn new(request_id: String, id: &'a str, details: T) -> Self {
        Self {
            request_id,
            id,
            details,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClientSubscriptionUpdate<'a, T> {
    #[serde(rename = "request-id")]
    id: &'a str,
    details: T,
}

impl<'a, T> ClientSubscriptionUpdate<'a, T> {
    pub fn new(id: &'a str, details: T) -> Self {
        Self { id, details }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SelectAccountResponsePayload {
    status: bool,
    #[serde(rename = "account-id")]
    account_id: String,
    #[serde(rename = "brokerage-id")]
    brokerage_id: String,
}

pub fn make_select_account_response<'a>(
    request_id: String,
    account_id: String,
    brokerage_id: String,
    status: bool,
) -> ClientMessage<'a, ClientResponse<'a, SelectAccountResponsePayload>> {
    ClientMessage::new_response(ClientResponse::new(
        request_id,
        "select-account",
        SelectAccountResponsePayload {
            account_id,
            brokerage_id,
            status,
        },
    ))
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountsSubscriptionUpdatePayload {
    accounts: Vec<BrokerageAccount>,
}

pub fn make_accounts_subscription_update<'a>(
    accounts: Vec<BrokerageAccount>,
) -> ClientMessage<'a, ClientSubscriptionUpdate<'a, AccountsSubscriptionUpdatePayload>> {
    ClientMessage::new_subscription_update(ClientSubscriptionUpdate::new(
        "accounts",
        AccountsSubscriptionUpdatePayload { accounts },
    ))
}
