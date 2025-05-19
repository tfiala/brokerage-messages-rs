use super::common::*;
use brokerage_messages::server_messages::*;
use rstest::fixture;

pub const REQUEST_ID: &str = "request_123";

#[fixture]
pub fn select_account_request<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = REQUEST_ID.to_string();
    let account_id = ACCOUNT_ID.to_string();
    let brokerage_id = BROKERAGE_ID.to_string();

    make_select_account_request::<'a>(request_id, account_id, brokerage_id)
}

#[fixture]
pub fn select_account_message<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = REQUEST_ID.to_string();
    let account_id = ACCOUNT_ID.to_string();
    let brokerage_id = BROKERAGE_ID.to_string();

    make_select_account_request::<'a>(request_id, account_id, brokerage_id)
}

#[fixture]
pub fn subscribe_accounts_message<'a>()
-> ServerMessage<'a, ServerSubscription<'a, SubscribeAccountsDetails>> {
    make_subscribe_accounts_message::<'a>()
}

#[fixture]
pub fn subscribe_account_ledger_message<'a>()
-> ServerMessage<'a, ServerSubscription<'a, SubscribeAccountLedgerDetails>> {
    make_subscribe_account_ledger_message::<'a>(ACCOUNT_ID.to_owned(), BROKERAGE_ID.to_owned())
}
