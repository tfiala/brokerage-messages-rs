mod fixtures;

use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
use fixtures::client_fixtures::*;
use fixtures::common::*;
use rstest::rstest;

//
// Select Account Response
//

#[rstest]
fn test_explicit_select_account_response_parsing(
    select_account_response: ClientMessage<ClientResponse<SelectAccountResponseDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_response).unwrap();
    let deserialized = serde_json::from_str::<
        ClientMessage<ClientResponse<SelectAccountResponseDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(
        deserialized,
        ClientMessage::new_response(ClientResponse::new(
            REQUEST_ID.to_owned(),
            "select-account",
            SelectAccountResponseDetails {
                account_id: select_account_response.data.details.account_id,
                brokerage_id: select_account_response.data.details.brokerage_id,
                status: select_account_response.data.details.status
            }
        ))
    );
}

#[rstest]
fn test_enum_select_account_response_parsing(
    select_account_response: ClientMessage<ClientResponse<SelectAccountResponseDetails>>,
) {
    let serialized = serde_json::to_string(&select_account_response).unwrap();
    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Response(EnumResponse {
            request_id: REQUEST_ID.to_owned(),
            details_enum: EnumResponseDetails::SelectAccount(SelectAccountResponseDetails {
                account_id: select_account_response.data.details.account_id,
                brokerage_id: select_account_response.data.details.brokerage_id,
                status: select_account_response.data.details.status
            })
        })
    );
}

//
// Accounts Subscription Update
//

#[rstest]
fn test_explicit_accounts_sub_update_parsing(accounts: Vec<BrokerageAccount>) {
    let message = make_accounts_subscription_update(accounts);
    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized = serde_json::from_str::<
        ClientMessage<ClientSubscriptionUpdate<AccountsSubUpdateDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(message, deserialized);
}

#[rstest]
fn test_enum_accounts_sub_update_parsing(accounts: Vec<BrokerageAccount>) {
    let message = make_accounts_subscription_update(accounts.clone());
    let serialized = serde_json::to_string(&message).unwrap();

    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Subscription(EnumSubDetails::Accounts(AccountsSubUpdateDetails {
            accounts
        }))
    );
}

//
// Account Ledger Subscription Update
//

#[rstest]
fn test_explicit_account_ledger_sub_update_parsing(
    account_ledger_update: ClientMessage<ClientSubscriptionUpdate<AccountLedgerSubUpdateDetails>>,
) {
    let serialized = serde_json::to_string(&account_ledger_update).unwrap();
    let deserialized = serde_json::from_str::<
        ClientMessage<ClientSubscriptionUpdate<AccountLedgerSubUpdateDetails>>,
    >(&serialized)
    .unwrap();

    assert_eq!(account_ledger_update, deserialized);
}

#[rstest]
fn test_enum_account_ledger_update_parsing(
    account_ledger_update: ClientMessage<ClientSubscriptionUpdate<AccountLedgerSubUpdateDetails>>,
) {
    let serialized = serde_json::to_string(&account_ledger_update).unwrap();
    let deserialized = serde_json::from_str::<EnumMessage>(&serialized).unwrap();

    assert_eq!(
        deserialized,
        EnumMessage::Subscription(EnumSubDetails::AccountLedger(
            AccountLedgerSubUpdateDetails {
                account_ledger: account_ledger_update.data.details.account_ledger
            }
        ))
    );
}
