mod fixtures;

// use brokerage_messages::domain::*;
use brokerage_messages::server_messages::*;
// use fixtures::common::*;
use fixtures::server_fixtures::*;
use rstest::rstest;

//
// Subscribe Accounts
//

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
fn test_enum_subscribe_accounts_parsing(
    subscribe_accounts_message: ServerMessage<ServerSubscription<SubscribeAccountsDetails>>,
) {
    let serialized = serde_json::to_string(&subscribe_accounts_message).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Subscription(EnumSubDetails::Accounts(SubscribeAccountsDetails {}))
    );
}

//
// Subscribe Account Ledger
//

#[rstest]
fn test_explicit_subscribe_account_ledger_parsing(
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
    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Subscription(EnumSubDetails::AccountLedger(
            SubscribeAccountLedgerDetails {
                account_id: subscribe_account_ledger_message.data.details.account_id,
                brokerage_id: subscribe_account_ledger_message.data.details.brokerage_id,
            }
        ))
    );
}

//
// Select Account Request
//

#[rstest]
fn test_explicit_select_account_request_parsing(
    select_account_request: ServerMessage<ServerRequest<SelectAccountDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_request).unwrap();
    let deserialized =
        serde_json::from_str::<ServerMessage<ServerRequest<SelectAccountDetails>>>(&serialized)
            .unwrap();

    assert_eq!(deserialized, select_account_request);
}

#[rstest]
fn test_enum_select_account_request_parsing(
    select_account_request: ServerMessage<ServerRequest<SelectAccountDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_request).unwrap();
    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Request(EnumRequestDetails::SelectAccount(SelectAccountDetails {
            account_id: select_account_request.data.details.account_id,
            brokerage_id: select_account_request.data.details.brokerage_id
        }))
    );
}
