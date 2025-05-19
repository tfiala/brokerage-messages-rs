mod fixtures;

use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
use brokerage_messages::server_messages::*;
use fixtures::*;
use rstest::rstest;

#[rstest]
fn test_accounts_sub_update_parsing(accounts: Vec<BrokerageAccount>) {
    let message = make_accounts_subscription_update(accounts);
    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized = serde_json::from_str::<
        ClientMessage<ClientSubscriptionUpdate<AccountsSubscriptionUpdatePayload>>,
    >(&serialized)
    .unwrap();

    assert_eq!(message, deserialized);
}

#[rstest]
fn test_explicit_select_account_request_parsing(
    select_account_request: ServerMessage<ServerRequest<SelectAccountDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_request).unwrap();
    let deserialized =
        serde_json::from_str::<ServerMessage<ServerRequest<SelectAccountDetails>>>(&serialized)
            .unwrap();

    assert_eq!(select_account_request, deserialized);
}

#[rstest]
fn test_request_enum_select_account_request_parsing(
    select_account_request: ServerMessage<ServerRequest<SelectAccountDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_request).unwrap();
    let deserialized =
        serde_json::from_str::<ServerMessage<ServerRequestDetails>>(&serialized).unwrap();

    assert_eq!(
        deserialized.data,
        ServerRequestDetails::SelectAccount(SelectAccountDetails {
            account_id: select_account_request.data.details.account_id,
            brokerage_id: select_account_request.data.details.brokerage_id
        })
    );
}
