use brokerage_messages::domain::*;
use rstest::fixture;

pub const ACCOUNT_ID: &str = "U1234567";
pub const BROKERAGE_ID: &str = "FIDELITY";

pub const ACCOUNT_ID_2: &str = "DU7654321";
pub const BROKERAGE_ID_2: &str = "IBKR";

pub const REQUEST_ID: &str = "request-123";

pub const ACCOUNT_TYPE: &str = "BROKERAGE-MARGIN";
pub const AVAILABLE_FUNDS: f64 = 100000.0;
pub const EQUITY_WITH_LOAN_VALUE: f64 = 200000.0;
pub const GROSS_POSITION_VALUE: f64 = 300000.0;
pub const CASH_BALANCE: f64 = 40000.0;

pub const NET_CASH_BALANCE: f64 = 50000.0;
pub const NET_STOCK_BALANCE: f64 = 60000.0;
pub const INTEREST: f64 = 7000.0;
pub const INTEREST_MTD: f64 = 8000.0;
pub const DIVIDENDS: f64 = 9000.0;
pub const DIVIDENDS_MTD: f64 = 10000.0;
pub const COMMISSIONS: f64 = 11000.0;
pub const COMMISSIONS_MTD: f64 = 12000.0;
pub const OTHER_FEES: f64 = 13000.0;
pub const OTHER_FEES_MTD: f64 = 14000.0;

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
