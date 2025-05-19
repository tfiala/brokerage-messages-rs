use brokerage_messages::domain::*;
use rstest::fixture;

pub const ACCOUNT_ID: &str = "U1234567";
pub const BROKERAGE_ID: &str = "FIDELITY";

pub const ACCOUNT_ID_2: &str = "DU7654321";
pub const BROKERAGE_ID_2: &str = "IBKR";

#[fixture]
pub fn accounts() -> Vec<BrokerageAccount> {
    vec![
        BrokerageAccount {
            account_id: ACCOUNT_ID.to_string(),
            brokerage_id: BROKERAGE_ID.to_string(),
        },
        BrokerageAccount {
            account_id: ACCOUNT_ID_2.to_string(),
            brokerage_id: BROKERAGE_ID_2.to_string(),
        },
    ]
}
