mod fixtures;

use brokerage_messages::client_messages::*;
use brokerage_messages::domain::*;
// use fixtures::client_fixtures::*;
use fixtures::common::*;
use rstest::rstest;

//
//
//

//
// Account Subscription Updates
//

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
