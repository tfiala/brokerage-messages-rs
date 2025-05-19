mod fixtures;

use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
use brokerage_messages::server_messages::*;
use fixtures::*;
use rstest::rstest;

#[rstest]
fn test_specific_accounts_sub_update_parsing(accounts: Vec<BrokerageAccount>) {
    let message = make_accounts_subscription_update(accounts);
    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized = serde_json::from_str::<
        ClientMessage<ClientSubscriptionUpdate<AccountsSubscriptionUpdateDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(message, deserialized);
}

#[rstest]
fn test_enum_accounts_sub_update_parsing(accounts: Vec<BrokerageAccount>) {
    let message = make_accounts_subscription_update(accounts.clone());

    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized: {}", serialized);

    let deserialized = serde_json::from_str::<ClientMessageEnum>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        ClientMessageEnum::Subscription(ClientSubscriptionDetails::Accounts(
            AccountsSubscriptionUpdateDetails { accounts }
        ))
    );
}

#[rstest]
fn test_specific_subscribe_account_ledger_parsing(
    subscribe_account_ledger_message: ServerMessage<
        ServerSubscription<SubscribeAccountLedgerDetails>,
    >,
) {
    let serialized = serde_json::to_string(&subscribe_account_ledger_message).unwrap();
    let deserialized = serde_json::from_str::<
        ServerMessage<ServerSubscription<SubscribeAccountLedgerDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(subscribe_account_ledger_message, deserialized);
}

#[rstest]
fn test_enum_subscribe_account_ledger_parsing(
    subscribe_account_ledger_message: ServerMessage<
        ServerSubscription<SubscribeAccountLedgerDetails>,
    >,
) {
    let serialized = serde_json::to_string(&subscribe_account_ledger_message).unwrap();
    let deserialized = serde_json::from_str::<ServerMessageEnum>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        ServerMessageEnum::Subscription(ServerSubscriptionDetails::AccountLedger(
            SubscribeAccountLedgerDetails {
                account_id: subscribe_account_ledger_message.data.details.account_id,
                brokerage_id: subscribe_account_ledger_message.data.details.brokerage_id,
            }
        ))
    );
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

#[rstest]
fn test_message_enum_select_account_request_parsing(
    select_account_request: ServerMessage<ServerRequest<SelectAccountDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_request).unwrap();
    let deserialized = serde_json::from_str::<ServerMessageEnum>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        ServerMessageEnum::Request(ServerRequestDetails::SelectAccount(SelectAccountDetails {
            account_id: select_account_request.data.details.account_id,
            brokerage_id: select_account_request.data.details.brokerage_id
        }))
    );
}

#[rstest]
fn test_explicit_subscribe_accounts_message_parsing(
    subscribe_accounts_message: ServerMessage<ServerSubscription<SubscribeAccountsDetails>>,
) {
    let serialized = serde_json::to_string(&subscribe_accounts_message).unwrap();
    let deserialized = serde_json::from_str::<
        ServerMessage<ServerSubscription<SubscribeAccountsDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(subscribe_accounts_message, deserialized);
}

#[rstest]
fn test_message_enum_subscribe_accounts_parsing(
    subscribe_accounts_message: ServerMessage<ServerSubscription<SubscribeAccountsDetails>>,
) {
    let serialized = serde_json::to_string(&subscribe_accounts_message).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized = serde_json::from_str::<ServerMessageEnum>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        ServerMessageEnum::Subscription(ServerSubscriptionDetails::Accounts(
            SubscribeAccountsDetails {}
        ))
    );
}
