use super::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]

pub struct ClientMessage<'a, T> {
    pub proto: &'a str,
    pub data: T,
}

impl<'a, T> ClientMessage<'a, T> {
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
    pub request_id: String,
    pub id: &'a str,
    pub details: T,
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
    pub id: &'a str,
    pub details: T,
}

impl<'a, T> ClientSubscriptionUpdate<'a, T> {
    pub fn new(id: &'a str, details: T) -> Self {
        Self { id, details }
    }
}

//
// Select Account Response
//

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SelectAccountResponseDetails {
    #[serde(rename = "account-id")]
    pub account_id: String,
    #[serde(rename = "brokerage-id")]
    pub brokerage_id: String,
    pub status: bool,
}

pub fn make_select_account_response<'a>(
    request_id: String,
    account_id: String,
    brokerage_id: String,
    status: bool,
) -> ClientMessage<'a, ClientResponse<'a, SelectAccountResponseDetails>> {
    ClientMessage::new_response(ClientResponse::new(
        request_id,
        "select-account",
        SelectAccountResponseDetails {
            account_id,
            brokerage_id,
            status,
        },
    ))
}

//
// Account Subscription Update
//

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountsSubUpdateDetails {
    pub accounts: Vec<BrokerageAccount>,
}

pub fn make_accounts_subscription_update<'a>(
    accounts: Vec<BrokerageAccount>,
) -> ClientMessage<'a, ClientSubscriptionUpdate<'a, AccountsSubUpdateDetails>> {
    ClientMessage::new_subscription_update(ClientSubscriptionUpdate::new(
        "accounts",
        AccountsSubUpdateDetails { accounts },
    ))
}

//
// Client Message Enum
//

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "details")]
pub enum EnumResponseDetails {
    #[serde(rename = "select-account")]
    SelectAccount(SelectAccountResponseDetails),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EnumResponse {
    #[serde(rename = "request-id")]
    pub request_id: String,

    #[serde(flatten)]
    pub details_enum: EnumResponseDetails,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "details")]
pub enum EnumSubDetails {
    #[serde(rename = "accounts")]
    Accounts(AccountsSubUpdateDetails),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "proto", content = "data")]
pub enum EnumMessage {
    #[serde(rename = "response")]
    Response(EnumResponse),
    #[serde(rename = "subscription")]
    Subscription(EnumSubDetails),
}
