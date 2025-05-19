use brokerage_messages::domain::*;
use brokerage_messages::server_messages::*;
use rstest::fixture;

#[fixture]
pub fn accounts() -> Vec<BrokerageAccount> {
    vec![
        BrokerageAccount {
            account_id: "123".to_string(),
            brokerage_id: "brokerage1".to_string(),
        },
        BrokerageAccount {
            account_id: "456".to_string(),
            brokerage_id: "brokerage2".to_string(),
        },
    ]
}

pub const ACCOUNT_ID: &str = "U1234567";
pub const BROKERAGE_ID: &str = "FIDELITY";

#[fixture]
pub fn select_account_request<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = "request_123".to_string();
    let account_id = ACCOUNT_ID.to_string();
    let brokerage_id = BROKERAGE_ID.to_string();

    make_select_account_request::<'a>(request_id, account_id, brokerage_id)
}

#[fixture]
pub fn select_account_message<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = "request_123".to_string();
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
