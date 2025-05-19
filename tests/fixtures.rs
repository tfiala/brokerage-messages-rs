use brokerage_messages::domain::*;
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
