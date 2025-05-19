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

#[fixture]
pub fn select_account_request<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = "request_123".to_string();
    let account_id = "123".to_string();
    let brokerage_id = "brokerage1".to_string();

    make_select_account_request::<'a>(request_id, account_id, brokerage_id)
}

#[fixture]
pub fn select_account_message<'a>() -> ServerMessage<'a, ServerRequest<'a, SelectAccountDetails>> {
    let request_id = "request_123".to_string();
    let account_id = "123".to_string();
    let brokerage_id = "brokerage1".to_string();

    make_select_account_request::<'a>(request_id, account_id, brokerage_id)
}
